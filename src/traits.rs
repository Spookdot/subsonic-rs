use serde::{Serialize, de::DeserializeOwned};

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
    type BasicResponse: DeserializeOwned;
    /// ReturnType for all other endpoints. Requires being able to take a Generic that implements
    /// [`DeserializeOwned`]
    /// 
    /// For example `T::SubsonicResponse<License>` for
    /// [`Client::get_license()`](crate::Client::get_license()) so it can return
    /// [`License`](crate::models::License)
    /// 
    /// See: [`SubsonicResponse`](crate::subsonic::models::SubsonicResponse) 
    /// and [`OpenSubsonicResponse`](crate::opensubsonic::models::OpenSubsonicResponse)
    type SubsonicResponse<T: DeserializeOwned>: DeserializeOwned + SubsonicResponseTrait<T>;
    /// ReturnType for the [`Client::search3()`](crate::Client::search3()) method
    ///
    /// See: [`subsonic::models::SearchResult3`](crate::subsonic::models::SearchResult3)
    /// and [`opensubsonic::models::SearchResult3`](crate::opensubsonic::models::SearchResult3)
    type SearchResult3: DeserializeOwned;
    /// ReturnType for the [`Client::search2()`](crate::Client::search2()) method
    ///
    /// See: [`subsonic::models::SearchResult2`](crate::subsonic::models::SearchResult2)
    /// and [`opensubsonic::models::SearchResult2`](crate::opensubsonic::models::SearchResult2)
    type SearchResult2: DeserializeOwned;
    /// Type used to represent songs in the API
    /// For example method [`Client::get_song()`](crate::Client::get_song())
    ///
    /// See: [`subsonic::models::Child`](crate::subsonic::models::Child)
    /// and [`opensubsonic::models::Child`](crate::opensubsonic::models::Child)
    type Child: DeserializeOwned;
}

// TODO rename Trait
/// Trait to be used for the SubsonicResponse associated Type in [`SubsonicServerInfo`]
///
/// See: [`SubsonicResponse`](crate::subsonic::models::SubsonicResponse) 
/// and [`OpenSubsonicResponse`](crate::opensubsonic::models::OpenSubsonicResponse)
pub trait SubsonicResponseTrait<T> {
    fn subsonic_response(&self) -> &impl SubsonicDataTrait<T>;
    fn into_subsonic_data(self) -> impl SubsonicDataTrait<T>;
}

// TODO rename Trait
/// Trait to be used by Types that would be wrapped by [`SubsonicResponseTrait`] implementors
///
/// See: [`SubsonicData`](crate::subsonic::models::SubsonicData) 
/// and [`OpenSubsonicData`](crate::opensubsonic::models::OpenSubsonicData)
pub trait SubsonicDataTrait<T> {
    fn status(&self) -> &str;
    fn version(&self) -> &str;
    fn additional(&self) -> &T;
    fn into_additional(self) -> T;
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
