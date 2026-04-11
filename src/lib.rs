use serde::Deserialize;
use rand::{distr::Alphanumeric, prelude::*};
use serde::Serialize;

// TODO Account for additional OpenSubsonic fields?
// And perhaps limit status to "ok" and "failed" Literals
// Perhaps make a variant for ok and a variant for failed where failed 
// can be all the different error codes
#[derive(Deserialize)]
pub struct SubsonicResponse {
    pub status: String,
    pub version: String
}

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
}

pub struct SubsonicClient {
    client: reqwest::Client,
    url: String,
    authentication: SubsonicAuthentication
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PingResponse {
    pub subsonic_response: SubsonicResponse
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
            version: String::from("1.16.0"), // TODO might wanna check if this is the way to go for the version
            format: String::from("json")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO Consider moving into .env file
    const URL: &str = "http://localhost:8081";
    const USERNAME: &str = "admin";
    const PASSWORD: &str = "adminpassword";

    #[tokio::test]
    async fn test_ping() {
        let subsonic_client = SubsonicClient::new(URL, USERNAME, PASSWORD);
        let ping_response = subsonic_client.ping().await;
        assert_eq!(ping_response.subsonic_response.status, String::from("ok"));
    }
}
