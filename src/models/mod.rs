use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SubsonicErrorCode {
    /// A generic error.
    GenericError = 0,
    /// Required parameter is missing.
    MissingParameter = 10,
    /// Incompatible Subsonic REST protocol version. Client must upgrade.
    ClientUpgradeRequired = 20,
    /// Incompatible Subsonic REST protocol version. Server must upgrade.
    ServerUpgradeRequired = 30,
    /// Wrong username or password.
    WrongLogin = 40,
    /// Token authentication not supported for LDAP users.
    NoTokenAuthForLdap = 41,
    /// Provided authentication mechanism not supported.
    AuthMechanismNotSupported = 42,
    /// Multiple conflicting authentication mechanisms provided.
    MultipleAuthMechanisms = 43,
    /// Invalid API key.
    InvalidApiKey = 44,
    /// User is not authorized for the given operation.
    NotAuthorized = 50,
    /// The trial period for the Subsonic server is over. Please upgrade to Subsonic Premium. Visit subsonic.org for details.
    TrialExpire = 60,
    /// The requested data was not found.
    RequestDataNotFound = 70,
}

/// A chatMessage.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::ChatMessage>(r#"
/// {
///   "username": "user",
///   "time": 1678935699000,
///   "message": "Api Script Testing"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    /// Username
    pub username: Box<str>,
    /// Time in millis since Jan 1 1970
    pub time: u64,
    /// The message
    pub message: Box<str>,
}

/// Chat messages list.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::ChatMessages>(r#"
/// {
///   "chatMessage": [
///     {
///       "username": "admin",
///       "time": 1678935707000,
///       "message": "Api Script Testing"
///     },
///     {
///       "username": "user",
///       "time": 1678935699000,
///       "message": "Api Script Testing"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessages {
    /// List of chatMessage
    #[serde(default)]
    pub chat_message: Vec<ChatMessage>
}

/// Artist details
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Artist>(r#"
/// {
///   "id": "100000002",
///   "name": "Synthetic",
///   "coverArt": "ar-100000002",
///   "starred": "2021-02-22T05:54:18Z"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    /// Artist id
    pub id: Box<str>,
    /// Artist name
    pub name: Box<str>,
    /// Artist image url
    pub artist_image_url: Option<Box<str>>,
    /// Artist starred date \[ISO 8601\]
    pub starred: Option<Box<str>>,
    /// Artist rating \[1-5\]
    pub user_rating: Option<u32>,
    /// Artist average rating \[1.0-5.0\]
    pub average_rating: Option<f32>
}

/// MusicFolder
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::MusicFolder>(r#"
/// {
///     "id": 4,
///     "name": "upload"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    /// The id
    pub id: u32,
    /// The folder name
    pub name: Option<Box<str>>
}

/// MusicFolders.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::MusicFolders>(r#"
/// {
///     "musicFolder": [
///         {
///             "id": 1,
///             "name": "music"
///         },
///         {
///             "id": 4,
///             "name": "upload"
///         }
///     ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolders {
    /// The folders
    #[serde(default)]
    pub music_folder: Vec<MusicFolder>
}

/// Lyrics.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Lyrics>(r#"
/// {
///   "artist": "Metallica",
///   "title": "Blitzkrieg",
///   "value": "Let us have peace, let us have life\n\nLet us escape the cruel night\n\nLet us have time, let the sun shine\n\nLet us beware the deadly sign\n\n\n\nThe day is coming\n\nArmageddon's near\n\nInferno's coming\n\nCan we survive the blitzkrieg?\n\nThe blitzkrieg\n\nThe blitzkrieg\n\n\n\nSave us from fate, save us from hate\n\nSave ourselves before it's too late\n\nCome to our need, hear our plea\n\nSave ourselves before the earth bleeds\n\n\n\nThe day is dawning\n\nThe time is near\n\nAliens calling\n\nCan we survive the blitzkrieg?"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    /// The lyrics
    pub value: Box<str>,
    /// The artist name
    #[serde(default)]
    pub artist: Box<str>,
    /// The song title
    #[serde(default)]
    pub title: Box<str>,
}

/// getLicense result.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::License>(r#"
/// {
///   "valid": true,
///   "email": "demo@demo.org",
///   "licenseExpires": "2017-04-11T10:42:50.842Z",
///   "trialExpires": "2017-04-11T10:42:50.842Z"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// The status of the license
    pub valid: bool,
    /// User email
    #[serde(default)]
    pub email: Box<str>,
    /// End of license date. \[ISO 8601\]
    #[serde(default)]
    pub license_expires: Box<str>,
    /// End of trial date. \[ISO 8601\]
    #[serde(default)]
    pub trial_expires: Box<str>,
}

