use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// Utility Trait to contain information on the specific API
pub trait SubsonicServerInfo {
    /// Struct containing authentication query parameters to be sent with each request to the API
    /// 
    /// See: [`subsonic::auth`](crate::auth)
    type SubsonicAuthentication: Serialize + SubsonicAuthenticationTrait;
    /// ReturnType for endpoints that do not send back any additional information
    /// For example [`Client::ping()`](crate::Client::ping())
    /// 
    /// See: [`SubsonicBasicResponse`](crate::subsonic::models::SubsonicBasicResponse) 
    /// and [`OpenSubsonicBasicResponse`](crate::opensubsonic::models::OpenSubsonicBasicResponse)
    type BasicResponse: Serialize + DeserializeOwned;
    /// ReturnType for all other endpoints. Requires being able to take a Generic that implements
    /// [`DeserializeOwned`]
    /// 
    /// For example `T::SubsonicResponse<License>` for
    /// [`Client::get_license()`](crate::Client::get_license()) so it can return
    /// [`License`](crate::models::License)
    /// 
    /// See: [`SubsonicResponse`](crate::subsonic::models::SubsonicResponse) 
    /// and [`OpenSubsonicResponse`](crate::opensubsonic::models::OpenSubsonicResponse)
    type SubsonicResponse<T: DeserializeOwned + Serialize + Debug>: DeserializeOwned + SubsonicResponseTrait<T, Self::ErrorData>;
    type ErrorData: ErrorDataTrait + Debug;
    /// Type used to represent songs in the API
    /// For example method [`Client::get_song()`](crate::Client::get_song())
    ///
    /// See: [`subsonic::models::Child`](crate::subsonic::models::Child)
    /// and [`opensubsonic::models::Child`](crate::opensubsonic::models::Child)
    type Child: Child + Serialize + DeserializeOwned + Debug;
    /// Type used to represent Playlists
    /// For example method [`Client::create_playlist()`](crate::Client::create_playlist())
    ///
    /// See: 
    /// See: [`subsonic::models::PlaylistWithSongs`](crate::subsonic::models::PlaylistWithSongs)
    /// and [`opensubsonic::models::PlaylistWithSongs`](crate::opensubsonic::models::PlaylistWithSongs)
    type PlaylistWithSongs: PlaylistWithSongs + Serialize + DeserializeOwned + Debug;
    type InternetRadioStation: InternetRadioStation + Serialize + DeserializeOwned + Debug;
    type Playlist: Playlist + Serialize + DeserializeOwned + Debug;
}

// TODO rename Trait
/// Trait to be used for the SubsonicResponse associated Type in [`SubsonicServerInfo`]
///
/// See: [`SubsonicResponse`](crate::subsonic::models::SubsonicResponse) 
/// and [`OpenSubsonicResponse`](crate::opensubsonic::models::OpenSubsonicResponse)
pub trait SubsonicResponseTrait<TData, TErrorData> {
    fn subsonic_response(&self) -> &impl SubsonicDataTrait<TData, TErrorData>;
    fn into_subsonic_data(self) -> impl SubsonicDataTrait<TData, TErrorData>;
}

// TODO rename Trait
/// Trait to be used by Types that would be wrapped by [`SubsonicResponseTrait`] implementors
///
/// See: [`SubsonicData`](crate::subsonic::models::SubsonicData) 
/// and [`OpenSubsonicData`](crate::opensubsonic::models::OpenSubsonicData)
pub trait SubsonicDataTrait<TData, TErrorData> {
    fn status(&self) -> &str;
    fn version(&self) -> &str;
    fn additional(&self) -> Result<&TData, &TErrorData>;
    fn into_additional(self) -> Result<TData, TErrorData>;
}

/// Trait to be implemented by structs containing Subsonic Authentication data to then be wrapped by
/// the [`SubsonicParameters`](crate::auth::SubsonicParameters) struct
///
/// See: [`SubsonicAuthentication`](crate::subsonic::auth::SubsonicAuthentication)
/// and [`OpenSubsonicAuthentication`](crate::opensubsonic::auth::OpenSubsonicAuthentication)
pub trait SubsonicAuthenticationTrait {
    fn legacy_password(username: &str, password: &str) -> Self;
    fn hashed_password(username: &str, password: &str) -> Self;
}

pub trait Child {}
pub trait Playlist {}
pub trait PlaylistWithSongs {}
pub trait InternetRadioStation {}

pub trait ErrorDataTrait {}
