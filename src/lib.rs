pub mod models;

use rand::{distr::Alphanumeric, prelude::*};
use serde::Serialize;
use crate::models::{PingResponse, SearchResult3Response};

// TODO Replace all that manually made models.rs code with a Rust generator based on the OpenAPI spec
// provided in this link here https://opensubsonic.netlify.app/docs/openapi/


#[derive(Serialize)]
pub struct SubsonicAuthentication {
    #[serde(rename = "u")]
    username: String,
    #[serde(rename = "t")]
    token: String,
    #[serde(rename = "s")]
    salt: String,
    #[serde(rename = "v")]
    version: String,
    #[serde(rename = "f")]
    format: String,
    #[serde(rename = "c")]
    client: String
}

pub struct SubsonicClient {
    client: reqwest::Client,
    url: String,
    authentication: SubsonicAuthentication
}

impl SubsonicClient {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let rng = rand::rng();
        let salt: String = rng.sample_iter(Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let salted_password = password.to_owned() + salt.as_str();
        let hashed_password = format!("{:x}", md5::compute(salted_password.as_bytes()));

        let authentication = SubsonicAuthentication {
            username: username.to_owned(),
            token: hashed_password.to_owned(),
            salt,
            version: String::from("1.16.1"), // TODO might wanna check if this is the way to go for the version
            format: String::from("json"),
            client: String::from("rust-subsonic-library") // TODO consider making this a parameter perhaps
        };

        Self {
            client: reqwest::Client::new(),
            url: url.to_owned(),
            authentication
        }
    }

    pub async fn ping(self) -> PingResponse {
        // TODO replace unwraps with actual error handling
        self.client.get(self.url + "/rest/ping.view")
            .query(&self.authentication)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn search3(self, query: &str) -> SearchResult3Response {
        // TODO replace unwraps with actual error handling
        // TODO move query parameters into a struct with additional options
        // Maybe add a Builder and a ::query factory that only takes the query parameter while you're at it
        self.client.get(self.url + "/rest/search3.view")
            .query(&self.authentication)
            .query(&[("query", query)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO Consider moving into .env file
    // const URL: &str = "http://localhost:8081";
    // const USERNAME: &str = "admin";
    // const PASSWORD: &str = "adminpassword";
    const URL: &str = "http://demo.subsonic.org";
    const USERNAME: &str = "guest4";
    const PASSWORD: &str = "guest";

    #[tokio::test]
    async fn test_ping() {
        let subsonic_client = SubsonicClient::new(URL, USERNAME, PASSWORD);
        let ping_response = subsonic_client.ping().await.subsonic_response;
        assert_eq!(ping_response.status, String::from("ok"), "{}", serde_json::to_string_pretty(&ping_response).unwrap());
    }

    #[tokio::test]
    async fn test_search3() {
        let subsonic_client = SubsonicClient::new(URL, USERNAME, PASSWORD);
        let query_response = subsonic_client.search3("Pyromantikk").await.subsonic_response;

        // Create string early in case of a failure down the line
        let serialized_response = serde_json::to_string_pretty(&query_response).unwrap();

        assert_eq!(query_response.status, String::from("ok"), "{serialized_response}");

        let search_result = query_response.search_result3;
        assert!(search_result.artist.is_none(), "{serialized_response}");

        assert_eq!(search_result.album.as_ref().unwrap().len(), 1, "{serialized_response}");
        assert_eq!(search_result.song.as_ref().unwrap().len(), 1, "{serialized_response}");
    }
}
