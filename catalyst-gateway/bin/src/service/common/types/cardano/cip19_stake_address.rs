//! Cardano address types.
//!
//! More information can be found in [CIP-19](https://cips.cardano.org/cip/CIP-19)

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

use anyhow::bail;
use const_format::concatcp;
use pallas::ledger::addresses::{Address, StakeAddress};
use poem_openapi::{
    registry::{MetaExternalDocument, MetaSchema, MetaSchemaRef},
    types::{Example, ParseError, ParseFromJSON, ParseFromParameter, ParseResult, ToJSON, Type},
};
use serde_json::Value;

use crate::service::common::types::string_types::impl_string_types;

/// Stake address title.
const TITLE: &str = "Cardano stake address";
/// Stake address description.
const DESCRIPTION: &str = "Cardano stake address, also known as a reward address.";
/// Stake address example.
// cSpell:disable
pub(crate) const EXAMPLE: &str = "stake_test1uqehkck0lajq8gr28t9uxnuvgcqrc6070x3k9r8048z8y5gssrtvn";
// cSpell:enable
/// Production Stake Address Identifier
const PROD_STAKE: &str = "stake";
/// Test Stake Address Identifier
const TEST_STAKE: &str = "stake_test";
/// Regex Pattern
pub(crate) const PATTERN: &str = concatcp!(
    "(",
    PROD_STAKE,
    "|",
    TEST_STAKE,
    ")1[a,c-h,j-n,p-z,0,2-9]{53}"
);
/// Length of the encoded address.
const ENCODED_ADDR_LEN: usize = 53;
/// Length of the decoded address.
const DECODED_ADDR_LEN: usize = 28;
/// Minimum length
pub(crate) const MIN_LENGTH: usize = PROD_STAKE.len() + 1 + ENCODED_ADDR_LEN;
/// Minimum length
pub(crate) const MAX_LENGTH: usize = TEST_STAKE.len() + 1 + ENCODED_ADDR_LEN;

/// String Format
pub(crate) const FORMAT: &str = "cardano:cip19-address";

/// External document for Cardano addresses.
static EXTERNAL_DOCS: LazyLock<MetaExternalDocument> = LazyLock::new(|| {
    MetaExternalDocument {
        url: "https://cips.cardano.org/cip/CIP-19".to_owned(),
        description: Some("CIP-19 - Cardano Addresses".to_owned()),
    }
});

/// Schema for `StakeAddress`.
static STAKE_SCHEMA: LazyLock<MetaSchema> = LazyLock::new(|| {
    MetaSchema {
        title: Some(TITLE.to_owned()),
        description: Some(DESCRIPTION),
        example: Some(Value::String(EXAMPLE.to_string())),
        external_docs: Some(EXTERNAL_DOCS.clone()),
        min_length: Some(MIN_LENGTH),
        max_length: Some(MAX_LENGTH),
        pattern: Some(PATTERN.to_string()),
        ..poem_openapi::registry::MetaSchema::ANY
    }
});

/// Because ALL the constraints are defined above, we do not ever need to define them in
/// the API. BUT we do need to make a validator.
/// This helps enforce uniform validation.
fn is_valid(stake_addr: &str) -> bool {
    // Just check the string can be safely converted into the type.
    if let Ok((hrp, addr)) = bech32::decode(stake_addr) {
        let hrp = hrp.as_str();
        addr.len() == DECODED_ADDR_LEN && (hrp == PROD_STAKE || hrp == TEST_STAKE)
    } else {
        false
    }
}

impl_string_types!(
    Cip19StakeAddress,
    "string",
    FORMAT,
    Some(STAKE_SCHEMA.clone()),
    is_valid
);

impl TryFrom<&str> for Cip19StakeAddress {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl TryFrom<String> for Cip19StakeAddress {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match bech32::decode(&value) {
            Ok((hrp, addr)) => {
                let hrp = hrp.as_str();
                if addr.len() == DECODED_ADDR_LEN && (hrp == PROD_STAKE || hrp == TEST_STAKE) {
                    return Ok(Cip19StakeAddress(value));
                }
                bail!("Invalid CIP-19 formatted Stake Address")
            },
            Err(err) => {
                bail!("Invalid CIP-19 formatted Stake Address : {err}");
            },
        };
    }
}

impl TryFrom<StakeAddress> for Cip19StakeAddress {
    type Error = anyhow::Error;

    fn try_from(addr: StakeAddress) -> Result<Self, Self::Error> {
        let addr_str = addr
            .to_bech32()
            .map_err(|e| anyhow::anyhow!(format!("Invalid stake address {e}")))?;
        Ok(Self(addr_str))
    }
}

impl TryInto<StakeAddress> for Cip19StakeAddress {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<StakeAddress, Self::Error> {
        let address_str = &self.0;
        let address = Address::from_bech32(address_str)?;
        match address {
            Address::Stake(address) => Ok(address),
            _ => Err(anyhow::anyhow!("Invalid stake address")),
        }
    }
}

impl Example for Cip19StakeAddress {
    fn example() -> Self {
        Self(EXAMPLE.to_owned())
    }
}