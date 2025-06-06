//! Consistent Pagination Types
//!
//! These types are paired and must be used together.
//!
//! Page - The Page we wish to request, defaults to 0.
//! Limit - The Limit we wish to request, defaults to 100.

use std::sync::LazyLock;

use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{Example, ParseError, ParseFromJSON, ParseFromParameter, ParseResult, ToJSON, Type},
};
use serde_json::Value;

//***** PAGE */
/// Page Title.
const PAGE_TITLE: &str = "Page";
/// Description.
macro_rules! page_description {
    () => {
        "The page number of the data.
The size of each page, and its offset within the complete data set is determined by the `limit` parameter."
    };
}
pub(crate) use page_description;
/// Description
pub(crate) const PAGE_DESCRIPTION: &str = page_description!();
/// Example.
const PAGE_EXAMPLE: u32 = 5;
/// Default
const PAGE_DEFAULT: u32 = 0;
/// Page Minimum.
const PAGE_MINIMUM: u32 = 0;
/// Page Maximum.
const PAGE_MAXIMUM: u32 = u32::MAX;

/// Schema.
#[allow(clippy::cast_precision_loss)]
static PAGE_SCHEMA: LazyLock<MetaSchema> = LazyLock::new(|| {
    MetaSchema {
        title: Some(PAGE_TITLE.to_owned()),
        description: Some(PAGE_DESCRIPTION),
        example: Some(PAGE_EXAMPLE.into()),
        default: Page(PAGE_DEFAULT).to_json(),
        maximum: Some(f64::from(PAGE_MAXIMUM)),
        minimum: Some(f64::from(PAGE_MINIMUM)),
        ..poem_openapi::registry::MetaSchema::ANY
    }
});

/// Page to be returned in the response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Page(u32);

impl Default for Page {
    fn default() -> Self {
        Self(PAGE_DEFAULT)
    }
}

/// Is the `Page` valid?
fn is_valid_page(value: u32) -> bool {
    (PAGE_MINIMUM..=PAGE_MAXIMUM).contains(&value)
}

impl Type for Page {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> std::borrow::Cow<'static, str> {
        "integer(u32)".into()
    }

    fn schema_ref() -> MetaSchemaRef {
        let schema_ref =
            MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("integer", "u32")));
        schema_ref.merge(PAGE_SCHEMA.clone())
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl ParseFromParameter for Page {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        let page: u32 = value.parse()?;
        Ok(Page(page))
    }
}

impl ParseFromJSON for Page {
    fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        if let Value::Number(value) = value {
            let value: u32 = value
                .as_u64()
                .and_then(|v| v.try_into().ok())
                .unwrap_or_default();
            if !is_valid_page(value) {
                return Err("invalid Page".into());
            }
            Ok(Self(value))
        } else {
            Err(ParseError::expected_type(value))
        }
    }
}

impl ToJSON for Page {
    fn to_json(&self) -> Option<Value> {
        Some(self.0.into())
    }
}

impl TryFrom<u32> for Page {
    type Error = anyhow::Error;

    fn try_from(page: u32) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            is_valid_page(page),
            "Invalid `page` value, must be in range {PAGE_MINIMUM}..{PAGE_MAXIMUM}"
        );
        Ok(Self(page))
    }
}

impl From<Page> for u32 {
    fn from(value: Page) -> Self {
        value.0
    }
}

impl Example for Page {
    fn example() -> Self {
        Self(PAGE_EXAMPLE)
    }
}

//***** LIMIT */
/// Title.
const LIMIT_TITLE: &str = "Limit";
/// Description - must be suitable for both the Query and Response docs.
macro_rules! limit_description {
    () => {
        "The size `limit` of each `page` of results.
Determines the maximum amount of data that can be returned in a valid response.

This `limit` of records of data will always be returned unless there is less data to return 
than allowed for by the `limit` and `page`.

*Exceeding the `page`/`limit` of all available records will not return `404`, it will return an 
empty response.*"
    };
}
pub(crate) use limit_description;
/// Description
pub(crate) const LIMIT_DESCRIPTION: &str = limit_description!();
/// Example.
const LIMIT_EXAMPLE: u32 = 10;
/// Default Limit (Should be used by paged responses to set the maximum size of the
/// response).
pub(crate) const LIMIT_DEFAULT: u32 = 100;
/// Minimum.
const LIMIT_MINIMUM: u32 = 1;
/// Maximum.
const LIMIT_MAXIMUM: u32 = LIMIT_DEFAULT;

/// Schema.
#[allow(clippy::cast_precision_loss)]
static LIMIT_SCHEMA: LazyLock<MetaSchema> = LazyLock::new(|| {
    MetaSchema {
        title: Some(LIMIT_TITLE.to_owned()),
        description: Some(LIMIT_DESCRIPTION),
        example: Some(LIMIT_EXAMPLE.into()),
        default: Page(LIMIT_DEFAULT).to_json(),
        maximum: Some(f64::from(LIMIT_MAXIMUM)),
        minimum: Some(f64::from(LIMIT_MINIMUM)),
        ..poem_openapi::registry::MetaSchema::ANY
    }
});

/// Limit of items to be returned in a page of data.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub(crate) struct Limit(u32);

impl Default for Limit {
    fn default() -> Self {
        Self(LIMIT_DEFAULT)
    }
}

/// Is the `Page` valid?
fn is_valid_limit(value: u32) -> bool {
    (LIMIT_MINIMUM..=LIMIT_MAXIMUM).contains(&value)
}

impl Type for Limit {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> std::borrow::Cow<'static, str> {
        "integer(u32)".into()
    }

    fn schema_ref() -> MetaSchemaRef {
        let schema_ref =
            MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("integer", "u32")));
        schema_ref.merge(LIMIT_SCHEMA.clone())
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl ParseFromParameter for Limit {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        let limit: u32 = value.parse()?;
        Ok(Limit(limit))
    }
}

impl ParseFromJSON for Limit {
    fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        if let Value::Number(value) = value {
            let value: u32 = value
                .as_u64()
                .and_then(|v| v.try_into().ok())
                .unwrap_or_default();
            if !is_valid_limit(value) {
                return Err("invalid Limit".into());
            }
            Ok(Self(value))
        } else {
            Err(ParseError::expected_type(value))
        }
    }
}

impl ToJSON for Limit {
    fn to_json(&self) -> Option<Value> {
        Some(self.0.into())
    }
}

impl TryFrom<u32> for Limit {
    type Error = anyhow::Error;

    fn try_from(limit: u32) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            is_valid_limit(limit),
            "Invalid `limit` value, must be in range {LIMIT_MINIMUM}..{LIMIT_MAXIMUM}"
        );
        Ok(Self(limit))
    }
}

impl From<Limit> for u32 {
    fn from(value: Limit) -> Self {
        value.0
    }
}

impl Example for Limit {
    fn example() -> Self {
        Self(LIMIT_EXAMPLE)
    }
}

//***** REMAINING : Not a Query Parameter, but tightly coupled type used in the pagination
//***** response. */
/// Title.
const REMAINING_TITLE: &str = "Remaining";
/// Description.
macro_rules! remaining_description {
    () => {
        "The number of items remaining to be returned after this page.
This is the absolute number of items remaining, and not the number of Pages."
    };
}
pub(crate) use remaining_description;
/// Description
pub(crate) const REMAINING_DESCRIPTION: &str = remaining_description!();
/// Example.
const REMAINING_EXAMPLE: u32 = 16_384;
/// Minimum.
const REMAINING_MINIMUM: u32 = 0;
/// Maximum.
const REMAINING_MAXIMUM: u32 = u32::MAX;

/// Schema.
#[allow(clippy::cast_precision_loss)]
static REMAINING_SCHEMA: LazyLock<MetaSchema> = LazyLock::new(|| {
    MetaSchema {
        title: Some(REMAINING_TITLE.to_owned()),
        description: Some(REMAINING_DESCRIPTION),
        example: Some(REMAINING_EXAMPLE.into()),
        maximum: Some(f64::from(REMAINING_MAXIMUM)),
        minimum: Some(f64::from(REMAINING_MINIMUM)),
        ..poem_openapi::registry::MetaSchema::ANY
    }
});

/// The remaining number of items to be returned after a page.
#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct Remaining(u32);

/// Is the `Remaining` valid?
fn is_valid_remaining(value: u32) -> bool {
    (REMAINING_MINIMUM..=REMAINING_MAXIMUM).contains(&value)
}

impl Type for Remaining {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> std::borrow::Cow<'static, str> {
        "integer(u32)".into()
    }

    fn schema_ref() -> MetaSchemaRef {
        let schema_ref =
            MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("integer", "u32")));
        schema_ref.merge(REMAINING_SCHEMA.clone())
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl ParseFromJSON for Remaining {
    fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
        let value = value.unwrap_or_default();
        if let Value::Number(value) = value {
            let value: u32 = value
                .as_u64()
                .and_then(|v| v.try_into().ok())
                .unwrap_or_default();
            if !is_valid_remaining(value) {
                return Err("invalid Remaining".into());
            }
            Ok(Self(value))
        } else {
            Err(ParseError::expected_type(value))
        }
    }
}

impl ToJSON for Remaining {
    fn to_json(&self) -> Option<Value> {
        Some(self.0.into())
    }
}

impl TryFrom<u32> for Remaining {
    type Error = anyhow::Error;

    fn try_from(remaining: u32) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            is_valid_remaining(remaining),
            "Invalid `remaining` value, must be in range {REMAINING_MINIMUM}..{REMAINING_MAXIMUM}"
        );
        Ok(Self(remaining))
    }
}

impl Remaining {
    /// Calculate remaining from total, page, limit, and the number of items returned.
    /// remaining = total - (page * limit) - items
    pub(crate) fn calculate(page: u32, limit: u32, total: u32, items: u32) -> Self {
        let remaining: u32 = total
            .saturating_sub(page.saturating_mul(limit))
            .saturating_sub(items);
        Self(remaining)
    }
}

impl From<Remaining> for u32 {
    fn from(value: Remaining) -> Self {
        value.0
    }
}

impl Example for Remaining {
    fn example() -> Self {
        Self(REMAINING_EXAMPLE)
    }
}
