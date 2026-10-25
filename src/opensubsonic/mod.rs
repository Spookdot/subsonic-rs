pub mod models;
pub mod auth;

use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{Client, SubsonicError};
use crate::traits::SubsonicServerInfo;
use models::*;
use auth::*;

/// Struct using the [`SubsonicServerInfo`] Trait to contain information specific to the OpenSubsonic
/// API
#[derive(Clone)]
pub struct OpenSubsonic;

impl SubsonicServerInfo for OpenSubsonic {
    type SubsonicAuthentication = OpenSubsonicAuthentication;
    type BasicResponse = OpenSubsonicBasicResponse;
    type SubsonicResponse<T: DeserializeOwned + Serialize + std::fmt::Debug> = OpenSubsonicResponse<T>;
    type ErrorData = ErrorData;
    type Child = Child;
    type PlaylistWithSongs = PlaylistWithSongs;
    type InternetRadioStation = InternetRadioStation;
    type Playlist = Playlist;
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
