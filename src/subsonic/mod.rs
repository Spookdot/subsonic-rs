pub mod models;
pub mod auth;

use serde::de::DeserializeOwned;
use crate::traits::SubsonicServerInfo;
use models::*;
use auth::*;

/// Struct using the [`SubsonicServerInfo`] Trait to contain information specific to the Subsonic
/// API
pub struct Subsonic;

impl SubsonicServerInfo for Subsonic {
    type SubsonicAuthentication = SubsonicAuthentication;
    type BasicResponse = SubsonicBasicResponse;
    type SubsonicResponse<T: DeserializeOwned> = SubsonicResponse<T>;
    type SearchResult3 = SearchResult3;
    type SearchResult2 = SearchResult2;
    type SearchResult = SearchResult;
    type Child = Child;
}

