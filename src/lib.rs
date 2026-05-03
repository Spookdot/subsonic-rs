pub mod models;

use rand::{distr::Alphanumeric, prelude::*};
use serde::Serialize;
use crate::models::*;
// use crate::types::SearchResult3;

// TODO Replace all that manually made models.rs code with a Rust generator based on the OpenAPI spec
// provided in this link here https://opensubsonic.netlify.app/docs/openapi/
// type SubsonicResponseSubsonicResponse = crate::models::SubsonicResponseSubsonicResponse<crate::models::Empty>;

/*
progenitor::generate_api!(
    spec = "openapi.json",
    pre_hook_async = crate::add_query_auth,
    post_hook_async = crate::debug_serde,
    inner_type = Arc<crate::SubsonicAuthentication>,
    replace = {
        AlbumId3 = crate::models::AlbumID3,
        Child = crate::models::Song,
        SubsonicSuccessResponse = crate::models::SubsonicSuccessResponse,
        SubsonicResponseSubsonicResponse = crate::models::SubsonicResponseSubsonicResponse<crate::models::Empty>,
    }
);


async fn add_query_auth(subsonic_authentication: &SubsonicAuthentication, req: &mut reqwest::Request) -> Result<(), serde_urlencoded::ser::Error> {
    let mut pairs = req.url_mut().query_pairs_mut();
    let serializer = serde_urlencoded::Serializer::new(&mut pairs);

    subsonic_authentication.serialize(serializer)?;
    Ok(())
}


async fn debug_serde(_: &SubsonicAuthentication, res: &Result<reqwest::Response, reqwest::Error>) -> Result<(), std::convert::Infallible> {
    if let Ok(response) = res {
        println!("{}", response.url());
    }
    Ok(())
}


// TODO Add the other Authorization methods, legacyPassword and apiToken
#[derive(Serialize, Debug)]
pub struct SubsonicAuthentication {
    #[serde(rename = "u")]
    username: Box<str>,
    #[serde(rename = "t")]
    token: Box<str>,
    #[serde(rename = "s")]
    salt: Box<str>,
    #[serde(rename = "v")]
    version: Box<str>,
    #[serde(rename = "f")]
    format: Box<str>,
    #[serde(rename = "c")]
    client: Box<str>
}

impl SubsonicAuthentication {
    pub fn new(username: &str, password: &str) -> Self {
        let rng = rand::rng();
        let salt: String = rng.sample_iter(Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let salted_password = password.to_owned() + salt.as_str();
        let hashed_password = format!("{:x}", md5::compute(salted_password.as_bytes()));

        Self {
            username: Box::from(username),
            token: Box::from(hashed_password),
            salt: Box::from(salt),
            version: Box::from("1.16.1"), // TODO might wanna check if this is the way to go for the version
            format: Box::from("json"),
            client: Box::from("rust-subsonic-library") // TODO consider making this a parameter perhaps
        }
    }
}

pub struct SubsonicClient {
    client: prelude::Client,
}

impl SubsonicClient {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let subsonic_authentication = Arc::from(SubsonicAuthentication::new(username, password));

        Self {
            client: Client::new(url, subsonic_authentication)
        }
    }

    pub async fn ping(&self) -> Result<crate::models::SubsonicSuccessResponse, SubsonicError> {
        let ping_response = self.client.ping().await?;
        // let ping_response: progenitor::progenitor_client::ResponseValue<types::SubsonicResponse> = match ping_result {
        //     Ok(e) => e,
        //     Err(_) => panic!(),
        // };

        // TODO Not sure I need to account for this?
        // if ping_response.status() != http::StatusCode::OK {
        //     panic!("Ping status was not okay. This needs to be handled!");
        // }

        // TODO Not sure *when* this happens or *if* it happens at all
        // if ping_response.subsonic_response.is_none() {
        //     return Err(SubsonicError::ResponseEmpty);
        // }
        // let ping_response = ping_response.subsonic_response.as_ref().unwrap();
        let ping_response = ping_response.into_inner().subsonic_response.ok_or(SubsonicError::ResponseEmpty)?;

        match ping_response {
            types::SubsonicResponseSubsonicResponse::SuccessResponse(response) => Ok(response),
            types::SubsonicResponseSubsonicResponse::FailureResponse(failure) => Err(SubsonicError::Failure(failure)),
            // types::SubsonicResponseSubsonicResponse::SuccessResponse(response) => Ok(response),
            // types::SubsonicResponseSubsonicResponse::FailureResponse(failure) => Err(SubsonicError::Failure(failure)),
        }
    }

    pub async fn search3(&self, query: &str) -> Result<SearchResult3, SubsonicError> {
        let search3_response = self.client.search3(None, None, None, None, None, query, None, None).await?;

        let search3_response = search3_response.into_inner().subsonic_response.ok_or(SubsonicError::ResponseEmpty)?;

        match search3_response {
            types::Search3ResponseSubsonicResponse::Search3SuccessResponse(response) => Ok(response.search_result3),
            types::Search3ResponseSubsonicResponse::SubsonicFailureResponse(failure) => Err(SubsonicError::Failure(failure)),
        }
    }

    pub async fn get_song(&self, id: &str) -> Result<crate::models::Song, SubsonicError> {
        let get_song_response = self.client.get_song(id).await?;
        
        let get_song_response = get_song_response.into_inner().subsonic_response.ok_or(SubsonicError::ResponseEmpty)?;

        match get_song_response {
            types::GetSongResponseSubsonicResponse::GetSongSuccessResponse(response) => Ok(response.song),
            types::GetSongResponseSubsonicResponse::SubsonicFailureResponse(failure) => Err(SubsonicError::Failure(failure)),
        }
    }
}
*/

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

/*
#[derive(Error, Debug)]
pub enum SubsonicError {
    // TODO
    #[error("Progenitor error that needs to be accounted for")]
    ProgenitorError(#[from] progenitor::progenitor_client::Error<()>),
    #[error("Subsonic gave an empty response")]
    ResponseEmpty,
    #[error("Subsonic returned a failure response")]
    Failure(types::SubsonicFailureResponse),
}
*/

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
    async fn ping(&self) -> PingResponse {
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

    // TODO Consider moving into .env file
    // const URL: &str = "http://localhost:8081";
    // const USERNAME: &str = "admin";
    // const PASSWORD: &str = "adminpassword";

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

    // #[tokio::test]
    // async fn test_search3() {
    //     let subsonic_client = SubsonicClient::new(URL, USERNAME, PASSWORD);
    //     let query_response = subsonic_client.search3("").await.subsonic_response;

    //     // Create string early in case of a failure down the line
    //     let serialized_response = serde_json::to_string_pretty(&query_response).unwrap();

    //     assert_eq!(query_response.status, String::from("ok"), "{serialized_response}");

    //     let search_result = query_response.search_result3;
    //     assert_eq!(search_result.artist.as_ref().unwrap().len(), 20, "{serialized_response}");
    //     assert_eq!(search_result.album.as_ref().unwrap().len(), 20, "{serialized_response}");
    //     assert_eq!(search_result.song.as_ref().unwrap().len(), 20, "{serialized_response}");
    // }

    // #[tokio::test]
    // async fn old_test_ping() {
    //     let subsonic_authentication = Arc::from(SubsonicAuthentication::new(USERNAME, PASSWORD));
    //     let subsonic_client = prelude::Client::new(URL, subsonic_authentication);

    //     let ping_response = subsonic_client.ping().await.unwrap();
    //     assert_eq!(ping_response.status(), http::StatusCode::OK);
    //     if ping_response.subsonic_response.is_none() {
    //         panic!("Subsonic Response is empty!");
    //     }
    //     let ping_response = ping_response.subsonic_response.as_ref().unwrap();
    //     if let types::SubsonicResponseSubsonicResponse::SuccessResponse(response) = ping_response {
    //         assert_eq!(response.status, "ok");
    //     } else {
    //         panic!("Subsonic responded with Failure!");
    //     }
    // }

    // #[tokio::test]
    // async fn old_test_search3() {
    //     let subsonic_authentication = Arc::from(SubsonicAuthentication::new(USERNAME, PASSWORD));
    //     let subsonic_client = prelude::Client::new(URL, subsonic_authentication);
    //     
    //     let search3_response = subsonic_client.search3(None, None, None, None, None, "", None, None).await.unwrap();
    //     assert_eq!(search3_response.status(), http::StatusCode::OK);
    //     
    //     let search3_response = search3_response.subsonic_response.as_ref().unwrap();
    //     if let types::Search3ResponseSubsonicResponse::Search3SuccessResponse(response) = search3_response {
    //         assert_eq!(response.status, Search3SuccessResponseStatus::Ok);
    //         println!("{}", serde_json::to_string(response).unwrap());
    //     } else {
    //         panic!("Subsonic responded with Failure!");
    //     }
    // }
    
    /*
    #[tokio::test]
    async fn test_ping() {
        let subsonic_client = SubsonicClient::new(SUBSONIC.url, SUBSONIC.username, SUBSONIC.password);

        let ping_response = subsonic_client.ping().await.unwrap();
        assert_eq!(ping_response.status, "ok");
    }

    #[tokio::test]
    async fn test_search3() {
        // Check it works for Subsonic Demo
        // TODO fix "Search3ResponseSubsonicResponse" requiring the extra OpenSubsonic fields
        // Using flatten "https://serde.rs/attr-flatten.html"
        // let subsonic_client = SubsonicClient::new(SUBSONIC.url, SUBSONIC.username, SUBSONIC.password);

        // let search3_response = subsonic_client.search3("Maya").await.unwrap();
        // assert_eq!(search3_response.artist.len(), 1);
        // assert_eq!(search3_response.album.len(), 1);
        // assert_eq!(search3_response.song.len(), 11);

        // Check it works for Navidrome demo
        let subsonic_client = SubsonicClient::new(NAVIDROME.url, NAVIDROME.username, NAVIDROME.password);

        let search3_response = subsonic_client.search3("").await.unwrap();
        assert_eq!(search3_response.artist.len(), 20);
        assert_eq!(search3_response.album.len(), 20);
        assert_eq!(search3_response.song.len(), 20);
    }

    #[tokio::test]
    async fn test_get_song() {
        // Check it works for Navidrome demo
        let subsonic_client = SubsonicClient::new(NAVIDROME.url, NAVIDROME.username, NAVIDROME.password);

        let search3_response = subsonic_client.search3("").await.unwrap();
        assert_eq!(search3_response.artist.len(), 20);
        assert_eq!(search3_response.album.len(), 20);
        assert_eq!(search3_response.song.len(), 20);
        
        let get_song_response = subsonic_client.get_song(&search3_response.song[0].id).await.unwrap();
        assert_eq!(search3_response.song[0].title, get_song_response.title);
    }
    */
}
