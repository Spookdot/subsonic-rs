pub mod models;
pub mod parameters;
pub mod auth;

#[cfg(test)]
mod tests;

use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use crate::models::*;
use crate::parameters::*;
use crate::auth::*;

#[derive(Error, Debug)]
pub enum SubsonicError {
    #[error("There was an error during an HTTP request")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Deserialization of a response failed")]
    SerdeError(#[from] serde_json::Error),
    #[error("The server returned a failed response")]
    Failed, // TODO actually contain any data from the server
    #[error("Deserialization failed at {url} for {body}")]
    Deserialization { url: Box<str>, body: Box<str>, serde_error: serde_json::Error },
}

pub trait SubsonicServerInfo {
    type SubsonicAuthentication: Serialize + SubsonicAuthenticationTrait;
    type BasicResponse: DeserializeOwned;
    type SubsonicResponse<T: DeserializeOwned>: DeserializeOwned + SubsonicResponseTrait<T>;
}
pub struct Subsonic;
pub struct OpenSubsonic;

impl SubsonicServerInfo for Subsonic {
    type SubsonicAuthentication = SubsonicAuthentication;
    type BasicResponse = SubsonicBasicResponse;
    type SubsonicResponse<T: DeserializeOwned> = SubsonicResponse<T>;
}

impl SubsonicServerInfo for OpenSubsonic {
    type SubsonicAuthentication = OpenSubsonicAuthentication;
    type BasicResponse = OpenSubsonicBasicResponse;
    type SubsonicResponse<T: DeserializeOwned> = OpenSubsonicResponse<T>;
}

/// A Client for the Subsonic API and OpenSubsonic
pub struct Client<T: SubsonicServerInfo> {
    client: reqwest::Client,
    url: String,
    parameters: SubsonicParameters<T::SubsonicAuthentication>,
    phantom: std::marker::PhantomData<T>
}
pub type SubsonicClient = Client<Subsonic>;
pub type OpenSubsonicClient = Client<OpenSubsonic>;

impl<T: SubsonicServerInfo> Client<T> {
    pub fn new(url: &str, parameters: SubsonicParameters<T::SubsonicAuthentication>) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: url.to_owned(),
            parameters,
            phantom: std::marker::PhantomData
        }
    }
    pub fn with_client(client: reqwest::Client, url: &str, parameters: SubsonicParameters<T::SubsonicAuthentication>) -> Self {
        Self {
            client,
            url: url.to_owned(),
            parameters,
            phantom: std::marker::PhantomData
        }
    }
    /// Used to test the connectivity with the server.
    pub async fn ping(&self) -> Result<T::BasicResponse, SubsonicError> {
        let url = format!("{}/rest/ping.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await?;

        Ok(response.json().await?)
    }
    pub async fn get_license(&self) -> Result<License, SubsonicError> {
        let url = format!("{}/rest/getLicense.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<License> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
    pub async fn search3(&self, parameters: Search3Parameters) -> Result<SearchResult3, SubsonicError> {
        let url = format!("{}/rest/search3.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        // TODO fix the type mess
        let url = response.url().to_owned();
        let response_text = response.text().await?;
        let subsonic_response: T::SubsonicResponse<SearchResult3> = match serde_json::from_str(&response_text) {
            Ok(success) => success,
            Err(error) => {
                return Err(SubsonicError::Deserialization { url: url.as_str().into(), body: response_text.into(), serde_error: error });
            }
        };

        let subsonic_data = subsonic_response.into_subsonic_data();
        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
    pub async fn star(&self, parameters: StarParameters) -> Result<T::BasicResponse, SubsonicError> {
        let url =  format!("{}/rest/star.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        Ok(response.json().await?)
    }
    pub async fn unstar(&self, parameters: StarParameters) -> Result<T::BasicResponse, SubsonicError> {
        let url =  format!("{}/rest/unstar.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        Ok(response.json().await?)
    }
    pub async fn get_song(&self, id: &str) -> Result<Song, SubsonicError> {
        let url = format!("{}/rest/getSong.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&[("id", id)])
            .send()
            .await?;

        // TODO fix the type mess
        let subsonic_response: T::SubsonicResponse<Song> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
    pub async fn get_lyrics(&self, parameters: GetLyricsParameters) -> Result<Lyrics, SubsonicError> {
        let url = format!("{}/rest/getLyrics.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<Lyrics> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        if subsonic_data.status() != "ok" {
            return Err(SubsonicError::Failed);
        }

        Ok(subsonic_data.into_additional())
    }
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
