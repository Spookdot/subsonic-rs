use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubsonicError {
    pub code: u8,
    pub message: String,
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

