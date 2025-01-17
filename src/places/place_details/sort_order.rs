//! Contains the `SortOrder` enum and its associated traits. It is used to
//! specify the sort order of reviews in the place details.

use crate::places::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------
//
/// The sorting method to use when returning reviews. Google recommends that you
/// display how the reviews are being sorted to the end user.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SortOrder {
    /// **Default** Reviews are sorted by relevance; the service will bias the
    /// results to return reviews originally written in the preferred language.
    #[serde(alias = "most_relevant")]
    MostRelevant,
    /// Reviews are sorted in chronological order; the preferred language does
    /// not affect the sort order.
    #[serde(alias = "newest")]
    Newest,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SortOrder {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match SortOrder::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&SortOrder> for String {
    /// Converts a `SortOrder` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from(field: &SortOrder) -> String {
        match field {
            SortOrder::MostRelevant => String::from("most_relevant"),
            SortOrder::Newest => String::from("newest"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static SORT_ORDER_TYPES_BY_CODE: phf::Map<&'static str, SortOrder> = phf_map! {
    // Basic
    "most_relevant" => SortOrder::MostRelevant,
    "newest" => SortOrder::Newest,
};

impl std::convert::TryFrom<&str> for SortOrder {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Error = crate::places::error::Error;
    /// Gets a `SortOrder` enum from a `String` that contains a supported
    /// [sort order](https://developers.google.com/maps/documentation/places/web-service/details#reviews_sort)
    /// code.
    fn try_from(sort_order_code: &str) -> Result<Self, Self::Error> {
        SORT_ORDER_TYPES_BY_CODE
            .get(sort_order_code)
            .cloned()
            .ok_or_else(|| Error::InvalidSortOrderCode(sort_order_code.to_string()))
    } // fn
} // impl

impl std::str::FromStr for SortOrder {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Err = crate::places::error::Error;
    /// Gets a `SortOrder` enum from a `String` that contains a supported
    /// [sort order](https://developers.google.com/maps/documentation/places/web-service/details#reviews_sort)
    /// code.
    fn from_str(sort_order_code: &str) -> Result<Self, Self::Err> {
        SORT_ORDER_TYPES_BY_CODE
            .get(sort_order_code)
            .cloned()
            .ok_or_else(|| Error::InvalidSortOrderCode(sort_order_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for SortOrder {
    /// Returns a reasonable default variant for the `SortOrder` enum type.
    fn default() -> Self {
        SortOrder::MostRelevant
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for SortOrder {
    /// Formats a `SortOrder` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SortOrder::MostRelevant => write!(f, "Most Relevant"),
            SortOrder::Newest => write!(f, "Newest"),
        } // match
    } // fn
} // impl