mod response;

pub use response::*;

use crate::models::SubsonicErrorCode;
use crate::traits::{self, ErrorDataTrait};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    pub code: SubsonicErrorCode,
    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub message: Box<str>,
}

impl ErrorDataTrait for ErrorData {}

/// Subsonic Response without any additional information to be wrapped
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::SubsonicBasicResponse>(r#"
/// {
///     "subsonic-response": {
///         "status": "ok",
///         "version": "1.16.1"
///     }
/// }
/// # "#).unwrap();
/// ```
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct SubsonicBasicResponse {
    /// Struct containing the actual data
    pub subsonic_response: SubsonicBasicData,
}

/// Subsonic Data without any additional information to be wrapped
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::SubsonicBasicData>(r#"
/// {
///     "status": "ok",
///     "version": "1.16.1"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicBasicData {
    /// The command result. `ok` or `failed`
    pub status: Box<str>,
    /// The server supported Subsonic API version.
    pub version: Box<str>
}

/// Playlist.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::Playlist>(r#"
/// {
///   "id": "800000075",
///   "name": "testcreate",
///   "owner": "user",
///   "public": true,
///   "created": "2023-03-16T03:18:41+00:00",
///   "changed": "2023-03-16T03:18:41+00:00",
///   "songCount": 1,
///   "duration": 304
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    /// Id of the playlist
    pub id: Box<str>,
    /// Name of the playlist
    pub name: Box<str>,
    /// A comment
    pub comment: Option<Box<str>>,
    /// Owner of the playlist
    pub owner: Option<Box<str>>,
    /// Is the playlist public
    pub public: Option<bool>,
    /// number of songs
    pub song_count: u32,
    /// Playlist duration in seconds
    pub duration: u32,
    /// Creation date \[ISO 8601\]
    pub created: Box<str>,
    /// Last changed date \[ISO 8601\]
    pub changed: Box<str>,
    /// A cover Art Id
    pub cover_art: Option<Box<str>>,
    /// A list of allowed usernames
    #[serde(default)]
    pub allowed_user: Vec<Box<str>>,
}

impl traits::Playlist for Playlist {}

/// Playlist with songs.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::PlaylistWithSongs>(r#"
/// {
///   "id": "800000075",
///   "name": "testcreate",
///   "owner": "user",
///   "public": true,
///   "created": "2023-03-16T03:18:41+00:00",
///   "changed": "2023-03-16T03:18:41+00:00",
///   "songCount": 1,
///   "duration": 304,
///   "entry": [
///     {
///       "id": "300000060",
///       "parent": "200000002",
///       "title": "BrownSmoke",
///       "isDir": false,
///       "isVideo": false,
///       "type": "music",
///       "albumId": "200000002",
///       "album": "Colorsmoke EP",
///       "artistId": "100000002",
///       "artist": "Synthetic",
///       "coverArt": "300000060",
///       "duration": 304,
///       "bitRate": 20,
///       "bitDepth": 16,
///       "samplingRate": 44100,
///       "channelCount": 2,
///       "userRating": 5,
///       "averageRating": 5,
///       "track": 4,
///       "year": 2007,
///       "genre": "Electronic",
///       "size": 792375,
///       "discNumber": 1,
///       "suffix": "wma",
///       "contentType": "audio/x-ms-wma",
///       "path": "Synthetic/Synthetic_-_Colorsmoke_EP-20k217-2007/04-Synthetic_-_BrownSmokeYSBM20k22khS.wma"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistWithSongs {
    /// Id of the playlist
    pub id: Box<str>,
    /// Name of the playlist
    pub name: Box<str>,
    /// A comment
    pub comment: Option<Box<str>>,
    /// Owner of the playlist
    pub owner: Option<Box<str>>,
    /// Is the playlist public
    pub public: Option<bool>,
    /// number of songs
    pub song_count: u32,
    /// Playlist duration in seconds
    pub duration: u32,
    /// Creation date \[ISO 8601\]
    pub created: Box<str>,
    /// Last changed date \[ISO 8601\]
    pub changed: Box<str>,
    /// A cover Art Id
    pub cover_art: Option<Box<str>>,
    /// A list of allowed usernames
    #[serde(default)]
    pub allowed_user: Vec<Box<str>>,
    /// The list of songs
    #[serde(default)]
    pub entry: Vec<Child>,
}

impl traits::PlaylistWithSongs for PlaylistWithSongs {}

/// An internetRadioStation.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::InternetRadioStation>(r#"
/// {
///   "id": "2",
///   "name": "HBR1.com - I.D.M. Tranceponder",
///   "streamUrl": "http://ubuntu.hbr1.com:19800/trance.ogg",
///   "homePageUrl": "http://www.hbr1.com/"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InternetRadioStation {
    /// The Id
    pub id: Box<str>,
    /// The name
    pub name: Box<str>,
    /// The radio url
    pub stream_url: Box<str>,
    /// The home page URL for the station.
    pub home_page_url: Option<Box<str>>,
}

impl traits::InternetRadioStation for InternetRadioStation {}

/// A media.
/// # Example
/// ```
/// # use subsonic::subsonic::models::Child;
/// # serde_json::from_str::<Child>(r#"
/// {
///   "id": "082f435a363c32c57d5edb6a678a28d4",
///   "parent": "e8a0685e3f3ec6f251649af2b58b8617",
///   "isDir": false,
///   "title": "\"polar expedition\"",
///   "album": "Live at The Casbah - 2005-04-29",
///   "artist": "The New Deal",
///   "track": 4,
///   "year": 2005,
///   "coverArt": "mf-082f435a363c32c57d5edb6a678a28d4_6410b3ce",
///   "size": 19866778,
///   "contentType": "audio/flac",
///   "suffix": "flac",
///   "starred": "2023-03-27T09:45:27Z",
///   "duration": 178,
///   "bitRate": 880,
///   "bitDepth": 16,
///   "samplingRate": 44100,
///   "channelCount": 2,
///   "path": "The New Deal/Live at The Casbah - 2005-04-29/04 - \"polar expedition\".flac",
///   "playCount": 8,
///   "discNumber": 1,
///   "created": "2023-03-14T17:51:22.112827504Z",
///   "albumId": "e8a0685e3f3ec6f251649af2b58b8617",
///   "artistId": "97e0398acf63f9fb930d7d4ce209a52b",
///   "type": "music",
///   "isVideo": false
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    /// The id of the media
    pub id: Box<str>,
    /// The id of the parent (folder/album)
    pub parent: Option<Box<str>>,
    /// The media is a directory
    pub is_dir: bool,
    /// The media name.
    pub title: Box<str>,
    /// The album name.
    pub album: Option<Box<str>>,
    /// The artist name.
    pub artist: Option<Box<str>>,
    /// The track number.
    pub track: Option<u16>,
    /// The media year.
    pub year: Option<u16>,
    /// The media genre
    pub genre: Option<Box<str>>,
    /// A covertArt id.
    pub cover_art: Option<Box<str>>,
    /// A file size of the media.
    pub size: Option<u32>,
    /// The mimeType of the media.
    pub content_type: Option<Box<str>>,
    /// The file suffix of the media.
    pub suffix: Option<Box<str>>,
    /// The transcoded mediaType if transcoding should happen.
    pub transcoded_content_type: Option<Box<str>>,
    /// The file suffix of the transcoded media.
    pub transcoded_suffice: Option<Box<str>>,
    /// The duration of the media in seconds.
    pub duration: Option<u32>,
    /// The bitrate of the media.
    pub bit_rate: Option<u16>,
    /// The full path of the media.
    pub path: Option<Box<str>>,
    /// Media is a video
    pub is_video: Option<bool>,
    /// The user rating of the media \[1-5\]
    pub user_rating: Option<u8>,
    /// The average rating of the media \[1.0-5.0\]
    pub average_rating: Option<f32>,
    /// The play count.
    pub play_count: Option<u32>,
    /// The disc number.
    pub disc_number: Option<u8>,
    /// Date the media was created. \[ISO 8601\]
    pub created: Option<DateTime<Utc>>,
    /// Date the media was starred. \[ISO 8601\]
    pub starred: Option<DateTime<Utc>>,
    /// The corresponding album id
    pub album_id: Option<Box<str>>,
    /// The corresponding artist id
    pub artist_id: Option<Box<str>>,
    /// The generic type of media \[music/podcast/audiobook/video\]
    #[serde(rename = "type")]
    pub song_type: Option<Box<str>>,
    /// The bookmark position in seconds
    pub bookmark_position: Option<u32>,
    /// The video original Width
    pub original_width: Option<u16>,
    /// The video original Height
    pub original_height: Option<u16>,
}

impl traits::Child for Child {}
