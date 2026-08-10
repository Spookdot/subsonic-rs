mod structured_lyrics;
mod response;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
pub use structured_lyrics::*;
pub use response::*;

use crate::models::Artist;

/// OpenSubsonic Response without any additional information to be wrapped
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::OpenSubsonicBasicResponse>(r#"
/// {
///     "subsonic-response": {
///         "status": "ok",
///         "version": "1.16.1",
///         "type": "AwesomeServerName",
///         "serverVersion": "0.1.3 (tag)",
///         "openSubsonic": true
///     }
/// }
/// # "#).unwrap();
/// ```
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct OpenSubsonicBasicResponse {
    /// Struct containing the actual data
    pub subsonic_response: OpenSubsonicBasicData,
}

/// OpenSubsonic Data without any additional information to be wrapped
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::OpenSubsonicBasicData>(r#"
/// {
///     "status": "ok",
///     "version": "1.16.1",
///     "type": "AwesomeServerName",
///     "serverVersion": "0.1.3 (tag)",
///     "openSubsonic": true
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenSubsonicBasicData {
    /// The command result. `ok` or `failed`
    pub status: Box<str>,
    /// The server supported Subsonic API version.
    pub version: Box<str>,
    /// Must return true if the server supports OpenSubsonic API v1
    pub open_subsonic: bool,
    /// The server actual version. \[Ex: `1.2.3 (beta)`\]
    pub server_version: Box<str>,
    /// The server actual name. \[Ex: `Navidrome` or `gonic`\]
    #[serde(rename = "type")]
    pub type_: Box<str>,
}

/// A supported OpenSubsonic API extension.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::OpenSubsonicExtension>(r#"
/// {
///     "name": "template",
///     "versions": [
///         1,
///         2
///     ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct OpenSubsonicExtension {
    /// The name of the extension
    pub name: Box<str>,
    /// The list of supported versions of this extension.
    pub versions: Vec<u32>,
}

/// searchResult
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::SearchResult>(r#"
/// {
///     "offset": 0,
///     "totalHits": 7,
///     "match": [
///         {
///             "id": "so-112",
///             "parent": "al-76",
///             "title": "Approaching Grave",
///             "isDir": false,
///             "isVideo": false,
///             "type": "music",
///             "albumId": "al-76",
///             "album": "...and a Dirge Becomes an Anthem",
///             "artistId": "ar-28",
///             "artist": "Crust",
///             "coverArt": "al-76",
///             "duration": 320,
///             "bitRate": 173,
///             "userRating": 5,
///             "playCount": 1939,
///             "created": "2020-10-29T08:56:50Z",
///             "starred": "2023-12-31T00:08:51Z",
///             "track": 1,
///             "year": 2020,
///             "size": 7460619,
///             "discNumber": 1,
///             "suffix": "mp3",
///             "contentType": "audio/mpeg",
///             "path": "/mnt/music/demo/music/a-757/01 Approaching Grave.mp3",
///             "artists": [
///                 {
///                     "id": "ar-28",
///                     "name": "Crust"
///                 }
///             ],
///             "albumArtists": [
///                 {
///                     "id": "ar-28",
///                     "name": "Crust"
///                 }
///             ]
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::SearchResult2>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::SearchResult3>(r#"
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
///       "played": "2023-03-28T00:45:13Z",
///       "created": "2023-03-10T02:19:35.784818075Z",
///       "artistId": "91c3901ac465b9efc439e4be4270c2b6",
///       "userRating": 4,
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::ArtistID3>(r#"
/// {
///   "id": "37ec820ca7193e17040c98f7da7c4b51",
///   "name": "2 Mello",
///   "coverArt": "ar-37ec820ca7193e17040c98f7da7c4b51_0",
///   "albumCount": 1,
///   "userRating": 5,
///   "artistImageUrl": "https://demo.org/image.jpg",
///   "starred": "2017-04-11T10:42:50.842Z",
///   "musicBrainzId": "189002e7-3285-4e2e-92a3-7f6c30d407a2",
///   "sortName": "Mello (2)",
///   "roles": [
///     "artist",
///     "albumartist",
///     "composer"
///   ]
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
    // OpenSubsonic Features
    /// The artist MusicBrainzID.
    pub music_brainz_id: Option<Box<str>>,
    /// The artist sort name.
    pub sort_name: Option<Box<str>>,
    /// A short human-readable comment, meant to be shown to users to distinguish between artists with the same name (e.g. `French electronic duo`).
    pub disambiguation: Option<Box<str>>,
    /// The list of all roles this artist has in the library.
    pub roles: Option<Vec<Box<str>>>,
}

/// An album from ID3 tags.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::AlbumID3>(r#"
/// {
///     "id": "ad0f112b6dcf83de5e9cae85d07f0d35",
///     "name": "8-bit lagerfeuer",
///     "version": "Deluxe Edition",
///     "artist": "pornophonique",
///     "year": 2007,
///     "coverArt": "al-ad0f112b6dcf83de5e9cae85d07f0d35_640a93a8",
///     "starred": "2023-03-22T01:51:06Z",
///     "duration": 1954,
///     "playCount": 97,
///     "genre": "Hip-Hop",
///     "created": "2023-03-10T02:19:35.784818075Z",
///     "artistId": "91c3901ac465b9efc439e4be4270c2b6",
///     "songCount": 8,
///     "played": "2023-03-28T00:45:13Z",
///     "userRating": 4,
///     "recordLabels": [
///         {
///             "name": "Sony"
///         }
///     ],
///     "musicBrainzId": "189002e7-3285-4e2e-92a3-7f6c30d407a2",
///     "genres": [
///         {
///             "name": "Hip-Hop"
///         },
///         {
///             "name": "East coast"
///         }
///     ],
///     "artists": [
///         {
///             "id": "ar-1",
///             "name": "Artist 1"
///         },
///         {
///             "id": "ar-2",
///             "name": "Artist 2"
///         }
///     ],
///     "displayArtist": "Artist 1 feat. Artist 2",
///     "releaseTypes": [
///         "Album",
///         "Remixes"
///     ],
///     "moods": [
///         "slow",
///         "cool"
///     ],
///     "sortName": "lagerfeuer (8-bit)",
///     "originalReleaseDate": {
///         "year": 2001,
///         "month": 3,
///         "day": 10
///     },
///     "releaseDate": {
///         "year": 2001,
///         "month": 3,
///         "day": 10
///     },
///     "isCompilation": false,
///     "explicitStatus": "explicit",
///     "discTitles": [
///         {
///             "disc": 0,
///             "title": "Disc 0 title",
///             "coverArt": "42"
///         },
///         {
///             "disc": 2,
///             "title": "Disc 1 title",
///             "coverArt": "6547"
///         }
///     ]
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
    /// Date the album was last played. \[ISO 8601\]
    pub played: Option<Box<str>>,
    /// The user rating of the album. \[1-5\]
    pub user_rating: Option<u8>,
    /// The labels producing the album.
    pub record_labels: Option<Vec<RecordLabel>>,
    /// The album MusicBrainzID.
    pub music_brainz_id: Option<Box<str>>,
    /// The list of all genres of the album.
    pub genres: Option<Vec<ItemGenre>>,
    /// The list of all album artists of the album.
    /// (Note: Only the required [`ArtistID3`] fields should be returned by default)
    pub artists: Option<Vec<ArtistID3>>,
    /// The single value display artist
    pub display_artist: Option<Box<str>>,
    /// The types of this album release. (Album, Compilation, EP, Remix, ...)
    pub release_types: Option<Vec<Box<str>>>,
    /// The list of all moods of the album.
    pub moods: Option<Vec<Box<str>>>,
    /// The album sort name.
    pub sort_name: Option<Box<str>>,
    /// Date the album was originally released.
    pub original_release_date: Option<ItemDate>,
    /// Date the specific edition of the album was released.
    /// Note: for files using ID3 tags, releaseDate should generally be read from the TDRL tag.
    /// Servers that use a different source for this field should document the behavior.
    pub release_date: Option<ItemDate>,
    /// True if the album is a compilation.
    pub is_compilation: Option<bool>,
    /// Returns "explicit" if at least one song is explicit, "clean" if no song is explicit and
    /// at least one is "clean" else "".
    pub explicit_status: Option<Box<str>>,
    /// The list of all disc titles of the album.
    pub disc_titles: Option<Vec<DiscTitle>>,
}

/// A media.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::Child>(r#"
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
///   "played": "2023-03-26T22:27:46Z",
///   "discNumber": 1,
///   "created": "2023-03-14T17:51:22.112827504Z",
///   "albumId": "e8a0685e3f3ec6f251649af2b58b8617",
///   "artistId": "97e0398acf63f9fb930d7d4ce209a52b",
///   "type": "music",
///   "mediaType": "song",
///   "isVideo": false,
///   "bpm": 134,
///   "comment": "This is a song comment",
///   "sortName": "Polar expedition",
///   "musicBrainzId": "189002e7-3285-4e2e-92a3-7f6c30d407a2",
///   "isrc": [
///     "USSM18300073",
///     "DELV42300297",
///     "DEE868300011",
///     "DEE868300007"
///   ],
///   "genres": [
///     {
///       "name": "Hip-Hop"
///     },
///     {
///       "name": "East coast"
///     }
///   ],
///   "artists": [
///     {
///       "id": "ar-1",
///       "name": "Artist 1"
///     },
///     {
///       "id": "ar-2",
///       "name": "Artist 2"
///     }
///   ],
///   "displayArtist": "Artist 1 feat. Artist 2",
///   "albumArtists": [
///     {
///       "id": "ar-6",
///       "name": "Artist 6"
///     },
///     {
///       "id": "ar-7",
///       "name": "Artist 7"
///     }
///   ],
///   "displayAlbumArtist": "Artist 6 & Artist 7",
///   "contributors": [
///     {
///       "role": "composer",
///       "artist": {
///         "id": "ar-3",
///         "name": "Artist 3"
///       }
///     },
///     {
///       "role": "composer",
///       "artist": {
///         "id": "ar-4",
///         "name": "Artist 4"
///       }
///     },
///     {
///       "role": "lyricist",
///       "artist": {
///         "id": "ar-5",
///         "name": "Artist 5"
///       }
///     },
///     {
///       "role": "performer",
///       "subRole": "Bass",
///       "artist": {
///         "id": "ar-5",
///         "name": "Artist 5"
///       }
///     }
///   ],
///   "displayComposer": "Artist 3, Artist 4",
///   "moods": [
///     "slow",
///     "cool"
///   ],
///   "explicitStatus": "explicit",
///   "replayGain": {
///     "trackGain": 0.1,
///     "albumGain": 1.1,
///     "trackPeak": 9.2,
///     "albumPeak": 9,
///     "baseGain": 0
///   },
///   "works": [
///     {
///       "name": "Symphony No. 5 in C minor, Op. 67",
///       "musicBrainzId": "d03bff61-26fc-301b-98ac-4d8e85771cbc"
///     }
///   ],
///   "movements": [
///     {
///       "name": "Andante con moto",
///       "number": 2,
///       "count": 4
///     }
///   ],
///   "groupings": ["Soundtrack", "Live"]
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
    /// The bit depth of the media.
    pub bit_depth: Option<u8>,
    /// The sampling rate of the media.
    pub sampling_rate: Option<u32>,
    /// The number of channels of the media.
    pub channel_count: Option<u8>,
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
    /// The actual media type \[song/album/artist\] 
    /// Note: If you support `musicBrainzId` you must support this field to ensure clients knows what the ID refers to.
    pub media_type: Option<Box<str>>,
    /// The bookmark position in seconds
    pub bookmark_position: Option<u32>,
    /// The video original Width
    pub original_width: Option<u16>,
    /// The video original Height
    pub original_height: Option<u16>,
    /// Date the album was last played. \[ISO 8601\]
    pub played: Option<DateTime<Utc>>,
    /// The BPM of the song.
    pub bpm: Option<u16>,
    /// The comment tag of the song.
    pub comment: Option<Box<str>>,
    /// The song sort name.
    pub sort_name: Option<Box<str>>,
    /// The track MusicBrainzID.
    pub music_brainz_id: Option<Box<str>>,
    /// The track ISRC(s).
    pub isrc: Option<Vec<Box<str>>>,
    /// The list of all genres of the song.
    pub genres: Option<Vec<ItemGenre>>,
    /// The list of all song artists of the song. 
    /// (Note: Only the required [`ArtistID3`] fields should be returned by default)
    pub artists: Option<Vec<ArtistID3>>,
    /// The single value display artist.
    pub display_artist: Option<Box<str>>,
    /// The list of all album artists of the song. 
    /// (Note: Only the required [`ArtistID3`] fields should be returned by default)
    pub album_artists: Option<Vec<ArtistID3>>,
    /// The single value display album artist.
    pub display_album_artist: Option<Box<str>>,
    /// The list of all contributor artists of the song.
    pub contributors: Option<Vec<Contributor>>,
    /// The single value display composer.
    pub display_composer: Option<Box<str>>,
    /// The list of all moods of the song.
    pub moods: Option<Vec<Box<str>>>,
    /// The replaygain data of the song.
    pub replay_gain: Option<ReplayGain>,
    /// Returns “explicit”, “clean” or “”. 
    /// (For songs extracted from tags “ITUNESADVISORY”: 1 = explicit, 2 = clean, MP4 “rtng”: 1 or 4 = explicit, 2 = clean. See `albumID3` for albums)
    pub explicit_status: Option<Box<str>>,
    /// The list of works associated with the song.
    pub works: Option<Vec<Work>>,
    /// The list of movements associated with the song.
    pub movements: Option<Vec<Movement>>,
    /// The list of groupings associated with the song.
    pub groupings: Option<Vec<Box<str>>>
}

/// A work associated with a song.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::Work>(r#"
/// {
///   "name": "Symphony No. 5 in C minor, Op. 67",
///   "musicBrainzId": "d03bff61-26fc-301b-98ac-4d8e85771cbc"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    /// The work name.
    pub name: Box<str>,
    /// The MusicBrainz Work ID.
    pub music_brainz_id: Option<Box<str>>,
}

/// A movement associated with a song.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::Movement>(r#"
/// {
///   "name": "Andante con moto",
///   "number": 2,
///   "count": 4
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    /// The movement name.
    pub name: Box<str>,
    /// The movement number.
    pub number: Option<u16>,
    /// The total number of movements.
    pub count: Option<u16>,
}

/// The replay gain data of a song.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::ReplayGain>(r#"
/// {
///     "trackGain": 0.1,
///     "albumGain": 1.1,
///     "trackPeak": 9.2,
///     "albumPeak": 0,
///     "baseGain": 0,
///     "fallbackGain": -8.1
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReplayGain {
    /// The track replay gain value. (In Db)
    pub track_gain: Option<f32>,
    /// The album replay gain value. (In Db)
    pub album_gain: Option<f32>,
    /// The track peak value. (Must be positive)
    pub track_peak: Option<f32>,
    /// The album peak value. (Must be positive)
    pub album_peak: Option<f32>,
    /// The base gain value. (In Db) (Ogg Opus Output Gain for example)
    pub base_gain: Option<f32>,
    /// An optional fallback gain that clients should apply when the
    /// corresponding gain value is missing.
    /// (Can be computed from the tracks or exposed as an user setting.)
    pub fallback_gain: Option<f32>,
}

/// A contributor artist for a or an album.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::Contributor>(r#"
/// {
///     "role": "performer",
///     "subRole": "Bass",
///     "artist": {
///         "id": "ar-1",
///         "name": "Artist 1"
///     }
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// The contributor role.
    pub role: Box<str>,
    /// The subRole for roles that may require it.
    /// Ex: The instrument for the performer role (TMCL/performer tags).
    /// Note: For consistency between different tag formats,
    /// the TIPL sub roles should be directly exposed in the role field.
    pub sub_role: Option<Box<str>>,
    /// The artist taking on the role.
    /// (Note: Only the required [ArtistID3] fields should be returned by default)
    pub artist: ArtistID3,
}

/// A genre in list of genres for an item
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::ItemGenre>(r#"
/// {
///   "name": "Noise"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ItemGenre {
    /// Genre name
    pub name: Box<str>,
}

/// A record label for an album.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::RecordLabel>(r#"
/// {
///   "name": "Sony"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecordLabel {
    /// The record label name.
    pub name: Box<str>,
}

/// A date for a media item that may be just a year, or year-month, or full date.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::ItemDate>(r#"
/// {
///   "year": 2020,
///   "month": 1,
///   "day": 1
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ItemDate {
    /// The year
    pub year: Option<u16>,
    /// The month (1-12)
    pub month: Option<u16>,
    /// The day (1-31)
    pub day: Option<u16>,
}

/// A disc title for an album, with an optional cover art.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::DiscTitle>(r#"
/// {
///   "disc": 0,
///   "title": "The disc title",
///   "coverArt": "65135"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiscTitle {
    /// The disc number.
    pub disc: u16,
    /// The nname of the disc.
    pub title: Box<str>,
    /// The cover art ID of the disc.
    pub cover_art: Option<Box<str>>,
}
