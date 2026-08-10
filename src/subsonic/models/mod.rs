mod response;

pub use response::*;
use crate::models::Artist;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

/// searchResult
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::SearchResult>(r#"
/// {
///     "offset" : 0,
///     "totalHits" : 41,
///     "match" : [ 
///         {
///             "id" : "302",
///             "parent" : "278",
///             "isDir" : false,
///             "title" : "PeerGynT - Old Tales",
///             "album" : "Broken Dreams",
///             "artist" : "PeerGynt Lobogris",
///             "track" : 10,
///             "year" : 2007,
///             "genre" : "Blues",
///             "coverArt" : "278",
///             "size" : 3846272,
///             "contentType" : "audio/mpeg",
///             "suffix" : "mp3",
///             "duration" : 237,
///             "bitRate" : 128,
///             "path" : "PeerGynt Lobogris/Broken Dreams/10 - 10 PeerGynT - Old Tales.mp3",
///             "playCount" : 943,
///             "created" : "2017-03-12T11:06:34.000Z",
///             "albumId" : "32",
///             "artistId" : "16",
///             "type" : "music"
///         } 
///     ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub offset: u32,
    pub total_hits: u32,
    #[serde(default, rename = "match")]
    pub match_: Vec<Child>,
}

/// searchResult2
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::SearchResult2>(r#"
/// {
///   "artist": [
///     {
///       "id": "100000002",
///       "name": "Synthetic",
///       "coverArt": "ar-100000002",
///       "starred": "2021-02-22T05:54:18Z"
///     }
///   ],
///   "album": [
///     {
///       "id": "200000021",
///       "parent": "100000036",
///       "album": "Forget and Remember",
///       "title": "Forget and Remember",
///       "name": "Forget and Remember",
///       "isDir": true,
///       "coverArt": "al-200000021",
///       "songCount": 20,
///       "created": "2021-07-22T02:09:31+00:00",
///       "duration": 4248,
///       "playCount": 0,
///       "artistId": "100000036",
///       "artist": "Comfort Fit",
///       "year": 2005,
///       "genre": "Hip-Hop"
///     }
///   ],
///   "song": [
///     {
///       "id": "082f435a363c32c57d5edb6a678a28d4",
///       "parent": "e8a0685e3f3ec6f251649af2b58b8617",
///       "isDir": false,
///       "title": "\"polar expedition\"",
///       "album": "Live at The Casbah - 2005-04-29",
///       "artist": "The New Deal",
///       "track": 4,
///       "year": 2005,
///       "coverArt": "mf-082f435a363c32c57d5edb6a678a28d4_6410b3ce",
///       "size": 19866778,
///       "contentType": "audio/flac",
///       "suffix": "flac",
///       "starred": "2023-03-27T09:45:27Z",
///       "duration": 178,
///       "bitRate": 880,
///       "bitDepth": 16,
///       "samplingRate": 44100,
///       "channelCount": 2,
///       "path": "The New Deal/Live at The Casbah - 2005-04-29/04 - \"polar expedition\".flac",
///       "playCount": 8,
///       "discNumber": 1,
///       "created": "2023-03-14T17:51:22.112827504Z",
///       "albumId": "e8a0685e3f3ec6f251649af2b58b8617",
///       "artistId": "97e0398acf63f9fb930d7d4ce209a52b",
///       "type": "music",
///       "isVideo": false
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SearchResult2 {
    /// Starred artists
    #[serde(default)]
    pub artist: Vec<Artist>,
    /// Starred albums
    #[serde(default)]
    pub album: Vec<Child>,
    /// Starred songs
    #[serde(default)]
    pub song: Vec<Child>,
}

/// search3 Result.
/// # Example
/// ```
/// # use subsonic::subsonic::models::SearchResult3;
/// # serde_json::from_str::<SearchResult3>(r#"
/// {
///   "artist": [
///     {
///       "id": "37ec820ca7193e17040c98f7da7c4b51",
///       "name": "2 Mello",
///       "coverArt": "ar-37ec820ca7193e17040c98f7da7c4b51_0",
///       "albumCount": 1,
///       "userRating": 5,
///       "artistImageUrl": "https://demo.org/image.jpg"
///     }
///   ],
///   "album": [
///     {
///       "id": "ad0f112b6dcf83de5e9cae85d07f0d35",
///       "name": "8-bit lagerfeuer",
///       "artist": "pornophonique",
///       "year": 2007,
///       "coverArt": "al-ad0f112b6dcf83de5e9cae85d07f0d35_640a93a8",
///       "starred": "2023-03-22T01:51:06Z",
///       "duration": 1954,
///       "playCount": 97,
///       "created": "2023-03-10T02:19:35.784818075Z",
///       "artistId": "91c3901ac465b9efc439e4be4270c2b6",
///       "songCount": 8
///     }
///   ],
///   "song": [
///     {
///       "id": "082f435a363c32c57d5edb6a678a28d4",
///       "parent": "e8a0685e3f3ec6f251649af2b58b8617",
///       "isDir": false,
///       "title": "\"polar expedition\"",
///       "album": "Live at The Casbah - 2005-04-29",
///       "artist": "The New Deal",
///       "track": 4,
///       "year": 2005,
///       "coverArt": "mf-082f435a363c32c57d5edb6a678a28d4_6410b3ce",
///       "size": 19866778,
///       "contentType": "audio/flac",
///       "suffix": "flac",
///       "starred": "2023-03-27T09:45:27Z",
///       "duration": 178,
///       "bitRate": 880,
///       "bitDepth": 16,
///       "samplingRate": 44100,
///       "channelCount": 2,
///       "path": "The New Deal/Live at The Casbah - 2005-04-29/04 - \"polar expedition\".flac",
///       "playCount": 8,
///       "played": "2023-03-26T22:27:46Z",
///       "discNumber": 1,
///       "created": "2023-03-14T17:51:22.112827504Z",
///       "albumId": "e8a0685e3f3ec6f251649af2b58b8617",
///       "artistId": "97e0398acf63f9fb930d7d4ce209a52b",
///       "type": "music",
///       "isVideo": false
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SearchResult3 {
    /// Matching artists
    #[serde(default)]
    pub artist: Vec<ArtistID3>,
    /// Matching albums
    #[serde(default)]
    pub album: Vec<AlbumID3>,
    /// Matching songs
    #[serde(default)]
    pub song: Vec<Child>,
}

/// An artist from ID3 tags.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::subsonic::models::ArtistID3>(r#"
/// {
///   "id": "37ec820ca7193e17040c98f7da7c4b51",
///   "name": "2 Mello",
///   "coverArt": "ar-37ec820ca7193e17040c98f7da7c4b51_0",
///   "albumCount": 1,
///   "userRating": 5,
///   "artistImageUrl": "https://demo.org/image.jpg",
///   "starred": "2017-04-11T10:42:50.842Z"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistID3 {
    /// The id of the artist.
    pub id: Box<str>,
    /// The artist name.
    pub name: Box<str>,
    /// A coverArt id.
    pub cover_art: Option<Box<str>>,
    /// An url to an external image source.
    pub artist_image_url: Option<Box<str>>,
    /// Artist album count.
    pub album_count: Option<u8>,
    /// Date the artist was starred. \[ISO 8601\]
    pub starred: Option<DateTime<Utc>>,
}

/// An album from ID3 tags.
/// # Example
/// ```
/// # use subsonic::subsonic::models::AlbumID3;
/// # serde_json::from_str::<AlbumID3>(r#"
/// {
///   "id": "ad0f112b6dcf83de5e9cae85d07f0d35",
///   "name": "8-bit lagerfeuer",
///   "artist": "pornophonique",
///   "year": 2007,
///   "coverArt": "al-ad0f112b6dcf83de5e9cae85d07f0d35_640a93a8",
///   "starred": "2023-03-22T01:51:06Z",
///   "duration": 1954,
///   "playCount": 97,
///   "genre": "Hip-Hop",
///   "created": "2023-03-10T02:19:35.784818075Z",
///   "artistId": "91c3901ac465b9efc439e4be4270c2b6",
///   "songCount": 8
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumID3 {
    /// The id of the album
    pub id: Box<str>,
    /// The album name.
    pub name: Box<str>,
    // OpenSubSonic Feature
    /// The album version name (Remastered, Anniversary Box Set, ...).
    pub version: Option<Box<str>>,
    /// Artist name.
    pub artist: Option<Box<str>>,
    /// The id of the artist
    pub artist_id: Option<Box<str>>,
    /// A coverArt id.
    pub cover_art: Option<Box<str>>,
    /// Number of songs
    pub song_count: u16,
    /// Total duration of the album in seconds
    pub duration: u16,
    /// Number of play of the album
    pub play_count: Option<u32>,
    /// Date the album was added. \[ISO 8601\]
    pub created: DateTime<Utc>,
    /// Date the album was starred. \[ISO 8601\]
    pub starred: Option<DateTime<Utc>>,
    /// The album year
    pub year: Option<u16>,
    /// The album genre
    pub genre: Option<Box<str>>,
}

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
