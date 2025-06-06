//! Catalyst RBAC Security Scheme
use std::{env, error::Error, sync::LazyLock, time::Duration};

use anyhow::{anyhow, Context};
use cardano_blockchain_types::{Network, Point, Slot, TxnIndex};
use cardano_chain_follower::ChainFollower;
use catalyst_types::id_uri::IdUri;
use ed25519_dalek::VerifyingKey;
use futures::{TryFutureExt, TryStreamExt};
use moka::future::Cache;
use poem::{error::ResponseError, http::StatusCode, IntoResponse, Request};
use poem_openapi::{auth::Bearer, payload::Json, SecurityScheme};
use rbac_registration::{
    cardano::cip509::{Cip509, RoleNumber},
    registration::cardano::RegistrationChain,
};
use tracing::{error, warn};

use super::token::CatalystRBACTokenV1;
use crate::{
    db::index::{
        queries::rbac::get_rbac_registrations::{Query, QueryParams},
        session::CassandraSession,
    },
    service::common::{
        responses::{
            code_500_internal_server_error::InternalServerError,
            code_503_service_unavailable::ServiceUnavailable, ErrorResponses,
        },
        types::headers::retry_after::RetryAfterHeader,
    },
};

/// Auth token in the form of catv1..
pub type EncodedAuthToken = String;

/// The header name that holds the authorization RBAC token
pub(crate) const AUTHORIZATION_HEADER: &str = "Authorization";

/// Cached auth tokens
// TODO: Caching is currently disabled because we want to measure the performance without it. See
// https://github.com/input-output-hk/catalyst-voices/issues/1940 for more details.
#[allow(dead_code)]
static CACHE: LazyLock<Cache<EncodedAuthToken, CatalystRBACTokenV1>> = LazyLock::new(|| {
    Cache::builder()
        // Time to live (TTL): 30 minutes
        .time_to_live(Duration::from_secs(30 * 60))
        // Time to idle (TTI):  5 minutes
        .time_to_idle(Duration::from_secs(5 * 60))
        // Create the cache.
        .build()
});

/// Catalyst RBAC Access Token
#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    key_name = "Authorization", // MUST match the `AUTHORIZATION_HEADER` constant.
    bearer_format = "catalyst-rbac-token",
    checker = "checker_api_catalyst_auth"
)]
#[allow(dead_code, clippy::module_name_repetitions)]
pub struct CatalystRBACSecurityScheme(CatalystRBACTokenV1);

impl From<CatalystRBACSecurityScheme> for IdUri {
    fn from(value: CatalystRBACSecurityScheme) -> Self {
        value.0.into()
    }
}

/// Error with the Authorization Token
///
/// We can not parse it, so its a 401 response.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Catalyst RBAC Auth Token")]
pub struct AuthTokenError;

impl ResponseError for AuthTokenError {
    fn status(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    /// Convert this error to a HTTP response.
    fn as_response(&self) -> poem::Response
    where Self: Error + Send + Sync + 'static {
        ErrorResponses::unauthorized().into_response()
    }
}

/// Token does not have required access rights
///
/// Not enough access rights, so its a 403 response.
#[derive(Debug, thiserror::Error)]
#[error("Insufficient Permission for Catalyst RBAC Token")]
pub struct AuthTokenAccessViolation(Vec<String>);

impl ResponseError for AuthTokenAccessViolation {
    fn status(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }

    /// Convert this error to a HTTP response.
    fn as_response(&self) -> poem::Response
    where Self: Error + Send + Sync + 'static {
        // TODO: Actually check permissions needed for an endpoint.
        ErrorResponses::forbidden(Some(self.0.clone())).into_response()
    }
}

/// Time in the past the Token can be valid for.
const MAX_TOKEN_AGE: Duration = Duration::from_secs(60 * 60); // 1 hour.

/// Time in the future the Token can be valid for.
const MAX_TOKEN_SKEW: Duration = Duration::from_secs(5 * 60); // 5 minutes

/// When added to an endpoint, this hook is called per request to verify the bearer token
/// is valid. The performed validation is described [here].
///
/// [here]: https://github.com/input-output-hk/catalyst-voices/blob/main/docs/src/catalyst-standards/permissionless-auth/auth-header.md#backend-processing-of-the-token
async fn checker_api_catalyst_auth(
    _req: &Request, bearer: Bearer,
) -> poem::Result<CatalystRBACTokenV1> {
    /// Temporary: Conditional RBAC for testing
    const RBAC_OFF: &str = "RBAC_OFF";

    // Deserialize the token: this performs the 1-5 steps of the validation.
    let token = CatalystRBACTokenV1::parse(&bearer.token).map_err(|e| {
        error!("Corrupt auth token: {e:?}");
        AuthTokenError
    })?;

    // If env var explicitly set by SRE, switch off full verification
    if env::var(RBAC_OFF).is_ok() {
        return Ok(token);
    };

    let registrations = indexed_registrations(token.catalyst_id()).await?;
    // Step 6: return 401 if the token isn't known.
    if registrations.is_empty() {
        error!(
            "Unable to find registrations for {} Catalyst ID",
            token.catalyst_id()
        );
        return Err(AuthTokenError.into());
    }

    // Step 7: Verify that the nonce is in the acceptable range.
    if !token.is_young(MAX_TOKEN_AGE, MAX_TOKEN_SKEW) {
        // Token is too old or too far in the future.
        error!("Auth token expired: {token}");
        Err(AuthTokenAccessViolation(vec!["EXPIRED".to_string()]))?;
    }

    // TODO: Caching is currently disabled because we want to measure the performance without
    // it.
    // // Its valid and young enough, check if its in the auth cache.
    // // This get() will extend the entry life for another 5 minutes.
    // // Even though we keep calling get(), the entry will expire
    // // after 30 minutes (TTL) from the origin insert().
    // // This is an optimization which saves us constantly looking up registrations we have
    // // already validated.
    // if let Some(token) = CACHE.get(&bearer.token).await {
    //     return Ok(token);
    // }

    // Step 8: get the latest stable signing certificate registered for Role 0.

    let public_key = last_signing_key(token.network(), &registrations)
        .await
        .map_err(|e| {
            error!(
                "Unable to get last signing key for {} Catalyst ID: {e:?}",
                token.catalyst_id()
            );
            AuthTokenError
        })?;

    // Step 9: Verify the signature.
    if let Err(error) = token.verify(&public_key) {
        error!(error=%error, "Invalid signature for token: {token}");
        Err(AuthTokenAccessViolation(vec![
            "INVALID SIGNATURE".to_string()
        ]))?;
    }

    // Step 10 is optional and isn't currently implemented.
    //   - Get the latest unstable signing certificate registered for Role 0.
    //   - Verify the signature against the Role 0 Public Key and Algorithm identified by the
    //     certificate. If this fails, return 403.

    // TODO: Caching is currently disabled because we want to measure the performance without
    // it.
    // // This entry will expire after 5 minutes (TTI) if there is no more ().
    // CACHE.insert(bearer.token, token.clone()).await;

    Ok(token)
}

/// Returns a sorted list of all registrations for the given Catalyst ID from the
/// database.
async fn indexed_registrations(catalyst_id: &IdUri) -> poem::Result<Vec<Query>> {
    let session = CassandraSession::get(true).ok_or_else(|| {
        error!("Failed to acquire db session");
        service_unavailable()
    })?;

    let mut result: Vec<_> = Query::execute(&session, QueryParams {
        catalyst_id: catalyst_id.clone().into(),
    })
    .and_then(|r| r.try_collect().map_err(Into::into))
    .await
    .map_err(|e| {
        error!("Failed to get RBAC registrations for {catalyst_id} Catalyst ID: {e:?}");
        if e.is::<bb8::RunError<tokio_postgres::Error>>() {
            service_unavailable()
        } else {
            let error = InternalServerError::new(None);
            error!(id=%error.id(), error=?e);
            ErrorResponses::ServerError(Json(error)).into()
        }
    })?;

    result.sort_by_key(|r| r.slot_no);
    Ok(result)
}

/// Returns a 503 error instance.
fn service_unavailable() -> poem::Error {
    let error = ServiceUnavailable::new(None);
    ErrorResponses::ServiceUnavailable(Json(error), Some(RetryAfterHeader::default())).into()
}

/// Returns the last signing key from the registration chain.
async fn last_signing_key(
    network: Network, indexed_registrations: &[Query],
) -> anyhow::Result<VerifyingKey> {
    let chain = registration_chain(network, indexed_registrations)
        .await
        .context("Failed to build registration chain")?;
    chain
        .get_latest_signing_pk_for_role(&RoleNumber::ROLE_0)
        .ok_or(anyhow!("Cannot find latest role 0 public key"))
        .map(|(pk, _)| pk)
}

/// Build a registration chain from the given indexed data.
async fn registration_chain(
    network: Network, indexed_registrations: &[Query],
) -> anyhow::Result<RegistrationChain> {
    let mut indexed_registrations = indexed_registrations.iter();
    let Some(root) = indexed_registrations.next() else {
        // We already checked that the registrations aren't empty, so we shouldn't get there.
        return Err(anyhow!("Empty registrations list"));
    };

    let root = registration(network, root.slot_no.into(), root.txn_index.into())
        .await
        .context("Failed to get root registration")?;
    let mut chain = RegistrationChain::new(root).context("Invalid root registration")?;

    for reg in indexed_registrations {
        // We only store valid registrations in this table, so an error here indicates a bug in
        // our indexing logic.
        let cip509 = registration(network, reg.slot_no.into(), reg.txn_index.into())
            .await
            .with_context(|| {
                format!(
                    "Invalid or missing registration at {:?} block {:?} transaction",
                    reg.slot_no, reg.txn_index,
                )
            })?;
        match chain.update(cip509) {
            Ok(c) => chain = c,
            Err(e) => {
                // This isn't a hard error because while the individual registration can be valid it
                // can be invalid in the context of the whole registration chain.
                warn!(
                    "Unable to apply registration from {:?} block {:?} txn index: {e:?}",
                    reg.slot_no, reg.txn_index
                );
            },
        }
    }

    Ok(chain)
}

/// Returns a RBAC registration from the given block and slot.
async fn registration(network: Network, slot: Slot, txn_index: TxnIndex) -> anyhow::Result<Cip509> {
    let point = Point::fuzzy(slot);
    let block = ChainFollower::get_block(network, point)
        .await
        .context("Unable to get block")?
        .data;
    if block.point().slot_or_default() != slot {
        // The `ChainFollower::get_block` function can return the next consecutive block if it
        // cannot find the exact one. This shouldn't happen, but we need to check anyway.
        return Err(anyhow!("Unable to find exact block"));
    }
    Cip509::new(&block, txn_index, &[])
        .context("Invalid RBAC registration")?
        .context("No RBAC registration at this block and txn index")
}
