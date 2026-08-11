//! # subsonic
//! Wrapper around both the Subsonic and OpenSubsonic API, intended to allow for wide support
//! through the use of Rust's complex type system
//!
//! Despite the large amount of structs this wrapper uses, we do not at the time have builders and
//! instead use a pattern of offering utility class methods and using struct literals together with
//! the Default Trait
//!
//! ## Class Method approach
//! ```rust
//! # use subsonic::parameters::Search3Parameters;
//! // This will fill all other fields with the defaults from the Default Trait unless otherwise
//! // specified
//! let search3_parameters = Search3Parameters::query("test");
//!
//! assert_eq!(search3_parameters.query, "test".into());
//! assert_eq!(search3_parameters.song_count, 20);
//! ```
//! The default in this case is a custom implementation. In those cases that will be specified with
//! the struct itself
//! 
//! ## Struct literal plus Default approach
//! ```rust
//! # use subsonic::parameters::Search3Parameters;
//! let search3_parameters = Search3Parameters { 
//!     query: "test".into(), 
//!     song_count: 0,
//!     ..Search3Parameters::default() 
//! };
//!
//! assert_eq!(search3_parameters.query, "test".into());
//! assert_eq!(search3_parameters.song_count, 0);
//! assert_eq!(search3_parameters.artist_count, 20);
//! ```

/// Return types for the different endpoints
pub mod models;
/// Parameter Structs for endpoints with more than one parameter
pub mod parameters;
/// Structs related to the authentication with Subsonic Servers
pub mod auth;
/// OpenSubsonic exclusive methods and its associated types
pub mod opensubsonic;
/// Subsonic exclusive methods and its associated types
pub mod subsonic;
/// Traits to be used when implementing other derivatives of Subsonic
pub mod traits;

#[cfg(test)]
mod tests;

use thiserror::Error;
use crate::models::*;
use crate::parameters::*;
use crate::traits::*;
use crate::auth::*;

#[derive(Error, Debug)]
pub enum SubsonicError<T: ErrorDataTrait> {
    #[error("There was an error during an HTTP request")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Deserialization of a response failed")]
    SerdeError(#[from] serde_json::Error),
    #[error("The server returned a failed response")]
    Failed(#[from] T), // TODO actually contain any data from the server
    #[error("Deserialization failed at {url} for {body}")]
    Deserialization { url: Box<str>, body: Box<str>, serde_error: serde_json::Error },
}

/// A Client for the Subsonic API and OpenSubsonic
pub struct Client<T: SubsonicServerInfo> {
    client: reqwest::Client,
    url: String,
    parameters: SubsonicParameters<T::SubsonicAuthentication>,
    phantom: std::marker::PhantomData<T>
}
/// Alias to [`Client`] struct with only Subsonic supported APIs
pub type SubsonicClient = Client<subsonic::Subsonic>;
/// Alias to [`Client`] struct with OpenSubsonic and Subsonic supported APIs
pub type OpenSubsonicClient = Client<opensubsonic::OpenSubsonic>;

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
    pub async fn ping(&self) -> Result<T::BasicResponse, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/ping.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await?;

        Ok(response.json().await?)
    }
    pub async fn get_license(&self) -> Result<License, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/getLicense.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<_> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    /// DEPRECATED endpoint included for compatibility reasons. 
    /// Please consider using [`Client::search2()`] or [`Client::search3()`] instead
    pub async fn search(&self, parameters: SearchParameters) -> Result<T::SearchResult, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/search.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<_> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn search2(&self, parameters: Search3Parameters) -> Result<T::SearchResult2, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/search2.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<_> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn search3(&self, parameters: Search3Parameters) -> Result<T::SearchResult3, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/search3.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        // TODO fix the type mess
        let url = response.url().to_owned();
        let response_text = response.text().await?;
        let subsonic_response: T::SubsonicResponse<_> = match serde_json::from_str(&response_text) {
            Ok(success) => success,
            Err(error) => {
                return Err(SubsonicError::Deserialization { url: url.as_str().into(), body: response_text.into(), serde_error: error });
            }
        };

        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn star(&self, parameters: StarParameters) -> Result<T::BasicResponse, SubsonicError<T::ErrorData>> {
        let url =  format!("{}/rest/star.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        Ok(response.json().await?)
    }
    pub async fn unstar(&self, parameters: StarParameters) -> Result<T::BasicResponse, SubsonicError<T::ErrorData>> {
        let url =  format!("{}/rest/unstar.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        Ok(response.json().await?)
    }
    pub async fn get_song(&self, id: &str) -> Result<T::Child, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/getSong.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&[("id", id)])
            .send()
            .await?;

        // TODO fix the type mess
        let subsonic_response: T::SubsonicResponse<_> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn get_lyrics(&self, parameters: GetLyricsParameters) -> Result<Lyrics, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/getLyrics.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<_> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn get_music_folders(&self) -> Result<MusicFolders, SubsonicError<T::ErrorData>> {
        let url = format!("{}/rest/getMusicFolders.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await?;

        
        let subsonic_response: T::SubsonicResponse<_> = response.json().await?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn create_user(&self, parameters: CreateUserParameters) -> Result<T::SubsonicResponse<()>, SubsonicError<T::ErrorData>> {
        let url =  format!("{}/rest/createUser.view", self.url);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(&parameters)
            .send()
            .await?;

        let subsonic_response: T::SubsonicResponse<()> = response.json().await?;
        if subsonic_response.subsonic_response().additional().is_err() {
            return Err(subsonic_response.into_subsonic_data().into_additional().err().unwrap().into());
        }

        Ok(subsonic_response)
    }
}
