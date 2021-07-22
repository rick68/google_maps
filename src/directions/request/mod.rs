//! **Look in this module for documentation on building your _Directions API_
//! query**. In particular, look at the _Request_ struct for examples of the
//! builder pattern. This module contains the tools (enums, structs, methods)
//! for building your Google Maps Platform request.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod new;
mod query_url;
mod validate;
mod with_alternatives;
mod with_arrival_time;
mod with_departure_time;
mod with_language;
mod with_region;
mod with_restrictions;
mod with_traffic_model;
mod with_transit_modes;
mod with_transit_route_preference;
mod with_travel_mode;
mod with_unit_system;
mod with_waypoint_optimization;
mod with_waypoints;
pub mod avoid;
pub mod departure_time;
pub mod location;
pub mod traffic_model;
pub mod transit_mode;
pub mod transit_route_preference;
pub mod unit_system;
pub mod waypoint;

use chrono::NaiveDateTime;
use crate::{
    client_settings::ClientSettings,
    directions::request::{
        avoid::Avoid,
        departure_time::DepartureTime,
        location::Location,
        traffic_model::TrafficModel,
        transit_mode::TransitMode,
        transit_route_preference::TransitRoutePreference,
        unit_system::UnitSystem,
        waypoint::Waypoint,
    }, // directions::request
    directions::travel_mode::TravelMode,
    language::Language,
    region::Region,
}; // crate

/// **Look at this `Request` struct for documentation on how to build your
/// _Directions API_ query**. The methods implemented for this struct are what's
/// used to build your request.

#[derive(Debug)]
pub struct Request<'a> {
    // Required parameters:
    // --------------------

    /// The address, latitude/longitude, or place ID to which you wish to
    /// calculate directions.
    destination: Location,

    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client_settings: &'a ClientSettings,

    /// The address, latitude/longitude, or place ID from which you wish to
    /// calculate directions.
    origin: Location,

    // Optional parameters:
    // --------------------

    /// Whether service may provide more than one route alternative in the
    /// response. See method `with_alternatives()` for more information.
    alternatives: Option<bool>,

    /// Desired arrival time. See method `with_arrival_time()` for more
    /// information.
    arrival_time: Option<NaiveDateTime>,

    /// Desired departure time. See files `departure_time.rs` and method
    /// `with_departure_time()` for more information.
    departure_time: Option<DepartureTime>,

    /// Language in which to return results. See file `language.rs` and method
    /// `with_language()` for more information.
    language: Option<Language>,

    /// Region bias. See file `region.rs` and method `with_region()` for more
    /// information.
    region: Option<Region>,

    /// Features that routes should avoid. See file `avoid.rs` and method
    /// `with_restrictions()` for more information.
    restrictions: Option<Vec<Avoid>>,

    /// Assumptions to use when calculating time in traffic. See file
    /// `traffic_model.rs` and method `with_traffic_model()` for more
    /// information.
    traffic_model: Option<TrafficModel>,

    /// Preferred modes of transit. See file `transit_mode.rs` and method
    /// `with_transit_modes()` for more information.
    transit_modes: Option<Vec<TransitMode>>,

    /// Preferences for transit routes. See file `transit_route_preference.rs`
    /// and method `with_transit_route_preference()` for more information.
    transit_route_preference: Option<TransitRoutePreference>,

    /// Mode of transportation. See file `travel_mode.rs` and method
    /// `with_travel_mode()` for more information.
    travel_mode: Option<TravelMode>,

    /// Unit system to use when displaying results. See file `unit_system.rs`
    /// and method `with_unit_system()` for more information.
    unit_system: Option<UnitSystem>,

    /// Whether the order of the intermediate locations should be optimized or
    /// not. See the method `with_waypoint_optimization()` for more information.
    waypoint_optimization: bool,

    /// Pass throughs or stopovers at intermediate locations. See file
    /// `waypoint.rs` and method `with_waypoints()` for more information.
    waypoints: Option<Vec<Waypoint>>,

    // Internal use only:
    // ------------------

    /// The URL-encoded query string that is passed to the Google Maps
    /// Directions API through cURL.
    query: Option<String>,

    /// Has the request been validated?
    validated: bool,
} // struct