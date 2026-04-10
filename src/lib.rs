use serde::Deserialize;

// TODO Account for additional OpenSubsonic fields?
// And perhaps limit status to "ok" and "failed" Literals
// Perhaps make a variant for ok and a variant for failed where failed 
// can be all the different error codes
#[derive(Deserialize)]
pub struct SubsonicResponse {
    pub status: String,
    pub version: String
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PingResponse {
    pub subsonic_response: SubsonicResponse
}

// TODO move ping into a full client that re-uses the reqwest client
/// Simple ping function to verify a working connection with a Subsonic server
pub async fn ping(url: &str, username: &str, password: &str) -> PingResponse {
    // TODO Replace hard coded salt with actual random salt
    let salt = "random_salt";

    let salted_password = password.to_owned() + salt;
    let hashed_password = format!("{:x}", md5::compute(salted_password.as_bytes()));

    let query_params = &[
        ("u", username),
        ("t", hashed_password.as_str()),
        ("s", salt),
        ("v", "1.16.0"), // TODO might wanna check if this is the way to go for the version
        ("f", "json")
    ];

    let client = reqwest::Client::new();
    // TODO replace unwraps with actual error handling
    client.post(url.to_owned() + "/rest/ping.view")
        .query(query_params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
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
        let ping_response = ping(URL, USERNAME, PASSWORD).await;
        assert_eq!(ping_response.subsonic_response.status, String::from("ok"));
    }
}
