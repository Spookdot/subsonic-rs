pub mod models;
pub mod auth;

use serde::de::DeserializeOwned;
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
    type SubsonicResponse<T: DeserializeOwned> = OpenSubsonicResponse<T>;
    type SearchResult3 = SearchResult3;
    type SearchResult2 = SearchResult2;
    type SearchResult = SearchResult;
    type Child = Child;
}

impl Client<OpenSubsonic> {
    pub async fn get_lyrics_by_song_id(&self, id: &str) -> Result<LyricsList, SubsonicError> {
        // TODO account for OpenSubsonic Servers that don't implement the Extension
        let url = format!("{}/rest/getLyricsBySongId.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&[("id", id)])
            .send()
            .await?;

        
        let subsonic_response: OpenSubsonicResponse<LyricsList> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
    pub async fn get_lyrics_by_song_id_enhanced(&self, id: &str) -> Result<EnhancedLyricsList, SubsonicError> {
        // TODO account for OpenSubsonic Servers that don't implement the Extension
        let url = format!("{}/rest/getLyricsBySongId.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&[("id", id), ("enhanced", "true")])
            .send()
            .await?;

        
        let subsonic_response: OpenSubsonicResponse<EnhancedLyricsList> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
    pub async fn get_open_subsonic_extensions(&self) -> Result<Vec<OpenSubsonicExtension>, SubsonicError> {
        let url = format!("{}/rest/getOpenSubsonicExtensions.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await?;

        
        let subsonic_response: OpenSubsonicResponse<Vec<OpenSubsonicExtension>> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
}
