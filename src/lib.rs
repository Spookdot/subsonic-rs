pub mod models;

use rand::{distr::Alphanumeric, prelude::*};
use serde::Serialize;
use crate::models::*;

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum SubsonicAuthentication {
    LegacyPassword {
        #[serde(rename = "u")]
        username: Box<str>,
        #[serde(rename = "p")]
        password: Box<str>,
    },
    HashedPassword {
        #[serde(rename = "u")]
        username: Box<str>,
        #[serde(rename = "t")]
        hashed_password: Box<str>,
        #[serde(rename = "s")]
        salt: Box<str>,
    },
    Token {
        #[serde(rename = "apiKey")]
        api_key: Box<str>
    }
}

#[derive(Serialize, Debug)]
pub struct SubsonicParameters {
    #[serde(rename = "v")]
    version: Box<str>,
    #[serde(rename = "f")]
    format: Box<str>,
    #[serde(rename = "c")]
    client: Box<str>,
    #[serde(flatten)]
    authentication: SubsonicAuthentication,
}

impl SubsonicParameters {
    fn new(client_name: &str, version: &str, authentication: SubsonicAuthentication) -> Self {
        Self {
            client: client_name.into(),
            version: version.into(),
            format: "json".into(),
            authentication,
        }
    }
    pub fn legacy_password(client_name: &str, username: &str, password: &str, version: &str) -> Self {
        let authentication = SubsonicAuthentication::LegacyPassword {
            username: username.into(),
            password: password.into(),
        };

        Self::new(client_name, version, authentication)
    }
    pub fn hashed_password(client_name: &str, username: &str, password: &str, version: &str) -> Self {
        let rng = rand::rng();
        let salt: String = rng.sample_iter(Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let salted_password = password.to_owned() + salt.as_str();
        let hashed_password = format!("{:x}", md5::compute(salted_password.as_bytes()));

        let authentication = SubsonicAuthentication::HashedPassword { 
            username: username.into(), 
            hashed_password: hashed_password.into(), 
            salt: salt.into() 
        };

        Self::new(client_name, version, authentication)
    }
    pub fn token(client_name: &str, token: &str, version: &str) -> Self {
        let authentication = SubsonicAuthentication::Token { api_key: token.into() };
        Self::new(client_name, version, authentication)
    }
}

pub struct Client {
    client: reqwest::Client,
    url: String,
    parameters: SubsonicParameters,
}

impl Client {
    pub fn new(url: &str, parameters: SubsonicParameters) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: url.to_owned(),
            parameters,
        }
    }
    pub async fn ping(&self) -> PingResponse {
        let url = self.url.clone() + "/rest/ping.view";
        let response = self.client.get(url)
            .query(&self.parameters)
            .send()
            .await
            .unwrap(); // TODO handle this error or at least delegate it

        response.json().await.unwrap() // TODO handle this error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SubsonicLogin<'a> {
        url: &'a str,
        username: &'a str,
        password: &'a str,
    }

    // Navidrome Demo (OpenSubsonic)
    const NAVIDROME: SubsonicLogin = SubsonicLogin { 
        url: "https://demo.navidrome.org", 
        username: "demo", 
        password: "demo" 
    };

    // Subsonic Demo
    const SUBSONIC: SubsonicLogin = SubsonicLogin { 
        url: "http://demo.subsonic.org", 
        username: "guest4", 
        password: "guest" 
    };

    #[tokio::test]
    async fn ping() {
        // For Subsonic
        let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
        let subsonic_client = Client::new(SUBSONIC.url, parameters);

        let ping_response = subsonic_client.ping().await;
        assert_eq!(ping_response.subsonic_response.status, "ok", "{}", serde_json::to_string_pretty(&ping_response).unwrap());

        // For Navidrome
        let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
        let subsonic_client = Client::new(NAVIDROME.url, parameters);

        let ping_response = subsonic_client.ping().await;
        assert_eq!(ping_response.subsonic_response.status, "ok", "{}", serde_json::to_string_pretty(&ping_response).unwrap());
    }
}
