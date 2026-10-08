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

use bytes::Bytes;
use futures::Stream;
use http::HeaderMap;
use http::HeaderValue;
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
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
    Failed(#[from] T),
    #[error("Deserialization failed at {url} for {body}")]
    Deserialization { url: Box<str>, body: Box<str>, serde_error: serde_json::Error },
    #[error("Content-Type Header missing in response")]
    ContentTypeHeaderMissing,
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
    async fn make_request<TParameter: Serialize>(
        &self,
        path: &str,
        parameters: &TParameter
    ) -> Result<reqwest::Response, SubsonicError<T::ErrorData>> {
        let url = format!("{}{}", self.url, path);
        let response = self.client.get(url)
            .query(&self.parameters)
            .query(parameters)
            .send()
            .await?;
        Ok(response)
    }
    fn verify_content_type(&self, headers: &HeaderMap<HeaderValue>) -> Result<bool, SubsonicError<T::ErrorData>> {
        let content_type = headers.get("content-type").ok_or(SubsonicError::ContentTypeHeaderMissing)?;
        Ok(content_type.to_str().unwrap().contains("application/json"))
    }
    async fn query<TParameter, TResponse>(
        &self, 
        path: &str, 
        parameters: &TParameter
    ) -> Result<TResponse, SubsonicError<T::ErrorData>> 
    where
        TParameter: Serialize, TResponse: DeserializeOwned + Serialize + std::fmt::Debug
    {
        let response = self.make_request(path, &parameters).await?;

        let text = response.text().await?;
        let subsonic_result = serde_json::from_str(text.as_str());

        // Pretty print the JSON in case serde can't parse it
        #[cfg(debug_assertions)]
        let subsonic_result = subsonic_result.inspect_err(|e| eprintln!("{:?}\n{:#?}", e, text));

        let subsonic_response: T::SubsonicResponse<_> = subsonic_result?;
        let subsonic_data = subsonic_response.into_subsonic_data();

        Ok(subsonic_data.into_additional()?)
    }
    pub async fn download(&self, id: &str) -> Result<impl Stream<Item = Result<Bytes, Error>>, SubsonicError<T::ErrorData>> {
        let response = self.make_request("/rest/download.view", &[("id", id)]).await?;

        if self.verify_content_type(response.headers())? {
            let data: T::SubsonicResponse<()> = response.json().await?;
            return Err(SubsonicError::Failed(data.into_subsonic_data().into_additional().unwrap_err()));
        }
        Ok(response.bytes_stream())
    }
    /// Used to test the connectivity with the server.
    pub async fn ping(&self) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/ping.view", &()).await
    }
    pub async fn get_license(&self) -> Result<License, SubsonicError<T::ErrorData>> {
        self.query("/rest/getLicense.view", &()).await
    }
    /// DEPRECATED endpoint included for compatibility reasons. 
    /// Please consider using [`Client::search2()`] or [`Client::search3()`] instead
    pub async fn search(&self, parameters: SearchParameters) -> Result<T::SearchResult, SubsonicError<T::ErrorData>> {
        self.query("/rest/search.view", &parameters).await
    }
    pub async fn search2(&self, parameters: Search3Parameters) -> Result<T::SearchResult2, SubsonicError<T::ErrorData>> {
        self.query("/rest/search2.view", &parameters).await
    }
    pub async fn search3(&self, parameters: Search3Parameters) -> Result<T::SearchResult3, SubsonicError<T::ErrorData>> {
        self.query("/rest/search3.view", &parameters).await
    }
    pub async fn star(&self, parameters: StarParameters) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/star.view", &parameters).await
    }
    pub async fn unstar(&self, parameters: StarParameters) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/unstar.view", &parameters).await
    }
    pub async fn get_song(&self, id: &str) -> Result<T::Child, SubsonicError<T::ErrorData>> {
        self.query("/rest/getSong.view", &[("id", id)]).await
    }
    pub async fn get_lyrics(&self, parameters: GetLyricsParameters) -> Result<Lyrics, SubsonicError<T::ErrorData>> {
        self.query("/rest/getLyrics.view", &parameters).await
    }
    pub async fn get_music_folders(&self) -> Result<MusicFolders, SubsonicError<T::ErrorData>> {
        self.query("/rest/getMusicFolders.view", &()).await
    }
    pub async fn create_user(&self, parameters: CreateUserParameters) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/createUser.view", &parameters).await
    }
    pub async fn delete_user(&self, username: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/deleteUser.view", &[("username", username)]).await
    }
    /// Adds a message to the chat log
    ///
    /// # Arguments
    /// * `message` - The chat message.
    pub async fn add_chat_message(&self, message: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/addChatMessage.view", &[("message", message)]).await
    }
    /// Return the current visible (non-expired) chat messages.
    ///
    /// # Arguments
    /// * `since` - Only return messages newer than this time (in millis since Jan 1 1970).
    pub async fn get_chat_messages(&self, since: Option<i32>) -> Result<ChatMessages, SubsonicError<T::ErrorData>> {
        self.query("/rest/getChatMessages.view", &[("since", since)]).await
    }
    pub async fn change_password(&self, username: &str, password: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query(
            "/rest/changePassword.view", 
            &[("username", username), ("password", password)]
        ).await
    }
    /// Creates or updates a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.
    pub async fn create_bookmark(&self, parameters: CreateBookmarkParameters) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/createBookmark.view", &parameters).await
    }
    /// Returns all bookmarks for this user. A bookmark is a position within a certain media file.
    pub async fn get_bookmarks(&self) -> Result<T::Bookmarks, SubsonicError<T::ErrorData>> {
        self.query("/rest/getBookmarks.view", &()).await
    }
    /// Creates or updates a bookmark (a position within a media file). Bookmarks are personal and not visible to other users.
    ///
    /// # Arguments
    /// * `id` - ID of the media file for which to delete the bookmark. Other users’ bookmarks are not affected.
    pub async fn delete_bookmark(&self, id: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/deleteBookmark.view", &[("id", id)]).await
    }
    /// Adds a new internet radio station. Only users with admin privileges are allowed to call this method.
    pub async fn create_internet_radio_station(&self, parameters: CreateInternetRadioStationParameters) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/createInternetRadioStation.view", &parameters).await
    }
    /// Returns all internet radio stations. Takes no extra parameters.
    pub async fn get_internet_radio_stations(&self) -> Result<T::InternetRadioStations, SubsonicError<T::ErrorData>> {
        self.query("/rest/getInternetRadioStations.view", &()).await
    }
    /// Deletes an existing internet radio station. Only users with admin privileges are allowed to call this method.
    ///
    /// # Arguments
    /// * `id` - The ID for the station.
    pub async fn delete_internet_radio_station(&self, id: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/deleteInternetRadioStation.view", &[("id", id)]).await
    }
    /// Creates (or updates) a playlist.
    pub async fn create_playlist(&self, parameters: CreatePlaylistParameters) -> Result<T::PlaylistWithSongs, SubsonicError<T::ErrorData>> {
        self.query("/rest/createPlaylist.view", &parameters.into_serializable()).await
    }
    /// Returns a listing of files in a saved playlist.
    ///
    /// # Arguments
    /// * `id` - ID of the playlist to return, as obtained by [`getPlaylists`](Client<T>::get_playlists()).
    pub async fn get_playlist(&self, id: &str) -> Result<T::PlaylistWithSongs, SubsonicError<T::ErrorData>> {
        self.query("/rest/getPlaylist.view", &[("id", id)]).await
    }
    /// Returns all playlists a user is allowed to play.
    ///
    /// # Arguments
    /// * `username` - (Since 1.8.0) If specified, return playlists for this user rather than for the authenticated user. 
    ///   The authenticated user must have admin role if this parameter is used.
    pub async fn get_playlists(&self, username: Option<&str>) -> Result<T::Playlists, SubsonicError<T::ErrorData>> {
        self.query("/rest/getPlaylists.view", &[("username", username)]).await
    }
    /// Deletes a saved playlist.
    ///
    /// # Arguments
    /// * `id` - ID of the playlist to delete, as obtained by
    ///   [`getPlaylists`](Client<T>::get_playlists()).
    pub async fn delete_playlist(&self, id: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/deletePlaylist.view", &[("id", id)]).await
    }
    /// Adds a new Podcast channel. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).
    ///
    /// # Arguments
    /// * `url` - The URL of the Podcast to add.
    pub async fn create_podcast_channel(&self, url: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/createPodcastChannel.view", &[("url", url)]).await
    }
    /// Deletes a Podcast channel. Note: The user must be authorized for Podcast administration (see Settings > Users > User is allowed to administrate Podcasts).
    ///
    /// # Arguments
    /// * `id` - The ID of the Podcast channel to delete.
    pub async fn delete_podcast_channel(&self, id: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/deletePodcastChannel.view", &[("id", id)]).await
    }
    /// Returns all Podcast channels the server subscribes to, and (optionally) their episodes.
    /// This method can also be used to return details for only one channel - refer to the `id` parameter.
    /// A typical use case for this method would be to first retrieve all channels without episodes,
    /// and then retrieve all episodes for the single channel the user selects.
    pub async fn get_podcasts(&self, parameters: GetPodcastsParameters) -> Result<Podcasts, SubsonicError<T::ErrorData>> {
        self.query("/rest/getPodcasts.view", &parameters).await
    }
    /// Creates a public URL that can be used by anyone to stream music or video from the server.
    /// The URL is short and suitable for posting on Facebook, Twitter etc.
    /// Note: The user must be authorized to share (see Settings > Users > User is allowed to share files with anyone).
    pub async fn create_share(&self, parameters: CreateShareParameters) -> Result<T::Shares, SubsonicError<T::ErrorData>> {
        self.query("/rest/createShare.view", &parameters.into_serializable()).await
    }
    /// Returns information about shared media this user is allowed to manage. Takes no extra parameters.
    pub async fn get_shares(&self) -> Result<T::Shares, SubsonicError<T::ErrorData>> {
        self.query("/rest/getShares.view", &()).await
    }
    /// Deletes an existing share.
    ///
    /// # Arguments
    /// * `id` - ID of the share to delete, as obtained by
    ///   [`getShares`](Client<T>::get_shares()).
    pub async fn delete_share(&self, id: &str) -> Result<(), SubsonicError<T::ErrorData>> {
        self.query("/rest/deleteShare.view", &[("id", id)]).await
    }
}
