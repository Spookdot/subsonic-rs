pub mod models;
pub mod auth;

use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{Client, SubsonicError};
use crate::traits::*;
use models::*;
use auth::*;

/// Struct using the [`SubsonicServerInfo`] Trait to contain information specific to the OpenSubsonic
/// API
pub struct OpenSubsonic;

impl SubsonicServerInfo for OpenSubsonic {
    type SubsonicAuthentication = OpenSubsonicAuthentication;
    type BasicResponse = OpenSubsonicBasicResponse;
    type SubsonicResponse<T: DeserializeOwned + Serialize> = OpenSubsonicResponse<T>;
    type ErrorData = ErrorData;
    type SearchResult3 = SearchResult3;
    type SearchResult2 = SearchResult2;
    type SearchResult = SearchResult;
    type Child = Child;
}

impl Client<OpenSubsonic> {
    pub async fn get_lyrics_by_song_id(&self, id: &str) -> Result<LyricsList, SubsonicError<ErrorData>> {
        // TODO account for OpenSubsonic Servers that don't implement the Extension
        self.query("/rest/getLyricsBySongId.view", &[("id", id)]).await
    }
    pub async fn get_lyrics_by_song_id_enhanced(&self, id: &str) -> Result<EnhancedLyricsList, SubsonicError<ErrorData>> {
        // TODO account for OpenSubsonic Servers that don't implement the Extension
        self.query("/rest/getLyricsBySongId.view", &[("id", id), ("enhanced", "true")]).await
    }
    pub async fn get_open_subsonic_extensions(&self) -> Result<Vec<OpenSubsonicExtension>, SubsonicError<ErrorData>> {
        self.query("/rest/getOpenSubsonicExtensions.view", &()).await
    }
}
