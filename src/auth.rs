use crate::traits::SubsonicAuthenticationTrait;
use rand::{distr::Alphanumeric, prelude::*};
use serde::Serialize;

pub(crate) fn hash_password(password: &str) -> (String, String) {
    let rng = rand::rng();
    let salt: String = rng.sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let salted_password = password.to_owned() + salt.as_str();
    let hashed_password = format!("{:x}", md5::compute(salted_password.as_bytes()));
    (hashed_password, salt)
}

/// Struct containing parameters added to every request to the API
#[derive(Serialize, Debug)]
pub struct SubsonicParameters<T: SubsonicAuthenticationTrait> {
    #[serde(rename = "v")]
    version: Box<str>,
    #[serde(rename = "f")]
    format: Box<str>,
    #[serde(rename = "c")]
    client: Box<str>,
    #[serde(flatten)]
    authentication: T,
}

impl<T: SubsonicAuthenticationTrait> SubsonicParameters<T> {
    pub(crate) fn new(client_name: &str, version: &str, authentication: T) -> Self {
        Self {
            client: client_name.into(),
            version: version.into(),
            format: "json".into(),
            authentication,
        }
    }
    /// Use the legacy password authentication method with a clear text password
    pub fn legacy_password(client_name: &str, username: &str, password: &str, version: &str) -> Self {
        // TODO implement hex-encoded variant
        let authentication = T::legacy_password(username, password);
        Self::new(client_name, version, authentication)
    }
    /// Supported since Subsonic 1.13.0
    ///
    /// Authenticate to Subsonic with a hashed password. Salt is generated inside the method. Uses
    /// md5
    pub fn hashed_password(client_name: &str, username: &str, password: &str, version: &str) -> Self {
        let authentication = T::hashed_password(username, password);
        Self::new(client_name, version, authentication)
    }
}
