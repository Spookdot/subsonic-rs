use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::traits::{Child, InternetRadioStation, Playlist};

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

/// An enumeration of possible podcast statuses.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Error,
    Deleted,
    Skipped
}

/// A Podcast episode.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::PodcastEpisode>(r#"
/// {
///   "id": "7390",
///   "parent": "7389",
///   "isDir": "false",
///   "title": "Jonas Gahr Støre",
///   "album": "NRK – Hallo P3",
///   "artist": "Podcast",
///   "year": "2015",
///   "coverArt": "7389",
///   "size": "41808585",
///   "contentType": "audio/mpeg",
///   "suffix": "mp3",
///   "duration": "2619",
///   "bitRate": "128",
///   "isVideo": "false",
///   "created": "2015-09-07T20:07:31.000Z",
///   "artistId": "453",
///   "type": "podcast",
///   "streamId": "7410",
///   "channelId": "17",
///   "description": "Jonas Gahr Støre fra Arbeiderpartiet er med i dagens partilederutspørring i Hallo P3!",
///   "status": "completed",
///   "publishDate": "2015-09-07T15:29:00.000Z"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisode {
    /// ID used for streaming podcast
    pub stream_id: Option<Box<str>>,
    /// ID of the podcast channel
    pub channel_id: Box<str>,
    /// Episode description
    pub description: Option<Box<str>>,
    /// Podcast status
    pub status: PodcastStatus,
    /// Date the episode was published [ISO 8601]
    pub publish_date: Option<Box<str>>
}

/// A Podcast channel.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::PodcastChannel>(r#"
/// {
///   "id": "1",
///   "url": "http://downloads.bbc.co.uk/podcasts/fivelive/drkarl/rss.xml",
///   "title": "Dr Karl and the Naked Scientist",
///   "description": "Dr Chris Smith aka The Naked Scientist with the latest news from the world of science and Dr Karl answers listeners' science questions.",
///   "coverArt": "pod-1",
///   "originalImageUrl": "http://downloads.bbc.co.uk/podcasts/fivelive/drkarl/drkarl.jpg",
///   "status": "completed",
///   "episode": [
///     {
///       "id": "34",
///       "streamId": "523",
///       "channelId": "1",
///       "title": "Scorpions have re-evolved eyes",
///       "description": "This week Dr Chris fills us in on the UK's largest free science festival, plus all this week's big scientific discoveries.",
///       "publishDate": "2011-02-03T14:46:43",
///       "status": "completed",
///       "parent": "11",
///       "isDir": "false",
///       "year": "2011",
///       "genre": "Podcast",
///       "coverArt": "24",
///       "size": "78421341",
///       "contentType": "audio/mpeg",
///       "suffix": "mp3",
///       "duration": "3146",
///       "bitRate": "128",
///       "path": "Podcast/drkarl/20110203.mp3"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodcastChannel {
    /// The channel ID
    pub id: Box<str>,
    /// Podcast channel URL
    pub url: Box<str>,
    /// The channel title
    pub title: Option<Box<str>>,
    /// The channel description
    pub description: Option<Box<str>>,
    /// ID used for retrieving cover art
    pub cover_art: Option<Box<str>>,
    /// URL for original image of podcast channel
    pub original_image_url: Option<Box<str>>,
    /// Channel status
    pub status: PodcastStatus,
    /// An error message
    pub error_message: Option<Box<str>>,
    /// Podcast episodes with this channel
    #[serde(default)]
    pub episode: Vec<PodcastEpisode>,
}

/// Podcasts.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Podcasts>(r#"
/// {
///   "channel": [
///     {
///       "id": "7390",
///       "url": "https://example.com/404",
///       "parent": "7389",
///       "isDir": "false",
///       "title": "Jonas Gahr Støre",
///       "album": "NRK – Hallo P3",
///       "artist": "Podcast",
///       "year": "2015",
///       "coverArt": "7389",
///       "size": "41808585",
///       "contentType": "audio/mpeg",
///       "suffix": "mp3",
///       "duration": "2619",
///       "bitRate": "128",
///       "isVideo": "false",
///       "created": "2015-09-07T20:07:31.000Z",
///       "artistId": "453",
///       "type": "podcast",
///       "streamId": "7410",
///       "channelId": "17",
///       "description": "Jonas Gahr Støre fra Arbeiderpartiet er med i dagens partilederutspørring i Hallo P3!",
///       "status": "completed",
///       "publishDate": "2015-09-07T15:29:00.000Z"
///     },
///     {
///       "id": "3",
///       "url": "https://example.com/404",
///       "status": "error",
///       "errorMessage": "Not Found"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Podcasts {
    /// Podcast channel(s)
    #[serde(default)]
    pub channel: Vec<PodcastChannel>
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

/// Share.
/// # Example
/// ```
/// # use subsonic::models::Share;
/// # use subsonic::opensubsonic::models::Child;
/// # serde_json::from_str::<Share<Child>>(r#"
/// {
///   "id": "12",
///   "url": "http://localhost:8989/share.php?id=12&secret=fXlKyEv3",
///   "description": "Forget and Remember (Comfort Fit)",
///   "username": "user",
///   "created": "2023-03-16T04:13:09+00:00",
///   "visitCount": 0,
///   "entry": [
///     {
///       "id": "300000116",
///       "parent": "200000021",
///       "title": "Can I Help U?",
///       "isDir": false,
///       "isVideo": false,
///       "type": "music",
///       "albumId": "200000021",
///       "album": "Forget and Remember",
///       "artistId": "100000036",
///       "artist": "Comfort Fit",
///       "coverArt": "300000116",
///       "duration": 103,
///       "bitRate": 216,
///       "bitDepth": 16,
///       "samplingRate": 44100,
///       "channelCount": 2,
///       "track": 1,
///       "year": 2005,
///       "genre": "Hip-Hop",
///       "size": 2811819,
///       "discNumber": 1,
///       "suffix": "mp3",
///       "contentType": "audio/mpeg",
///       "path": "user/Comfort Fit/Forget And Remember/1 - Can I Help U?.mp3"
///     },
///     {
///       "id": "300000121",
///       "parent": "200000021",
///       "title": "Planetary Picknick",
///       "isDir": false,
///       "isVideo": false,
///       "type": "music",
///       "albumId": "200000021",
///       "album": "Forget and Remember",
///       "artistId": "100000036",
///       "artist": "Comfort Fit",
///       "coverArt": "300000121",
///       "duration": 358,
///       "bitRate": 238,
///       "bitDepth": 16,
///       "samplingRate": 44100,
///       "channelCount": 2,
///       "track": 2,
///       "year": 2005,
///       "genre": "Hip-Hop",
///       "size": 10715592,
///       "discNumber": 1,
///       "suffix": "mp3",
///       "contentType": "audio/mpeg",
///       "path": "user/Comfort Fit/Forget And Remember/2 - Planetary Picknick.mp3"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Share<TChild: Child> {
    /// The share id
    pub id: Box<str>,
    /// The share url
    pub url: Box<str>,
    /// A description
    pub description: Option<Box<str>>,
    /// The username
    pub username: Box<str>,
    /// Creation date \[ISO 8601\]
    pub created: Box<str>,
    /// Share expiration \[ISO 8601\]
    pub expires: Option<Box<str>>,
    /// Last visit \[ISO 8601\]
    pub last_visited: Option<Box<str>>,
    /// Visit count
    pub visit_count: u32,
    /// A list of share
    #[serde(default = "Vec::new")]
    pub entry: Vec<TChild>,
}

/// Shares.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::opensubsonic::models::Shares>(r#"
/// {
///   "share": [
///     {
///       "id": "12",
///       "url": "http://localhost:8989/share.php?id=12&secret=fXlKyEv3",
///       "description": "Forget and Remember (Comfort Fit)",
///       "username": "user",
///       "created": "2023-03-16T04:13:09+00:00",
///       "visitCount": 0,
///       "entry": [
///         {
///           "id": "300000116",
///           "parent": "200000021",
///           "title": "Can I Help U?",
///           "isDir": false,
///           "isVideo": false,
///           "type": "music",
///           "albumId": "200000021",
///           "album": "Forget and Remember",
///           "artistId": "100000036",
///           "artist": "Comfort Fit",
///           "coverArt": "300000116",
///           "duration": 103,
///           "bitRate": 216,
///           "bitDepth": 16,
///           "samplingRate": 44100,
///           "channelCount": 2,
///           "track": 1,
///           "year": 2005,
///           "genre": "Hip-Hop",
///           "size": 2811819,
///           "discNumber": 1,
///           "suffix": "mp3",
///           "contentType": "audio/mpeg",
///           "path": "user/Comfort Fit/Forget And Remember/1 - Can I Help U?.mp3"
///         },
///         {
///           "id": "300000121",
///           "parent": "200000021",
///           "title": "Planetary Picknick",
///           "isDir": false,
///           "isVideo": false,
///           "type": "music",
///           "albumId": "200000021",
///           "album": "Forget and Remember",
///           "artistId": "100000036",
///           "artist": "Comfort Fit",
///           "coverArt": "300000121",
///           "duration": 358,
///           "bitRate": 238,
///           "bitDepth": 16,
///           "samplingRate": 44100,
///           "channelCount": 2,
///           "track": 2,
///           "year": 2005,
///           "genre": "Hip-Hop",
///           "size": 10715592,
///           "discNumber": 1,
///           "suffix": "mp3",
///           "contentType": "audio/mpeg",
///           "path": "user/Comfort Fit/Forget And Remember/2 - Planetary Picknick.mp3"
///         }
///       ]
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Shares<TChild: Child> {
    /// A list of share
    #[serde(default = "Vec::new")]
    pub share: Vec<Share<TChild>>
}

/// Playlist.
/// # Example
/// ```
/// # use subsonic::models::Playlists;
/// # use subsonic::opensubsonic::models::Playlist;
/// # serde_json::from_str::<Playlists<Playlist>>(r#"
/// {
///   "playlist": [
///     {
///       "id": "800000003",
///       "name": "random - admin - private (admin)",
///       "owner": "admin",
///       "public": false,
///       "created": "2021-02-23T04:35:38+00:00",
///       "changed": "2021-02-23T04:35:38+00:00",
///       "songCount": 43,
///       "duration": 17875
///     },
///     {
///       "id": "800000002",
///       "name": "random - admin - public (admin)",
///       "owner": "admin",
///       "public": true,
///       "created": "2021-02-23T04:34:56+00:00",
///       "changed": "2021-02-23T04:34:56+00:00",
///       "songCount": 43,
///       "duration": 17786
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Playlists<TPlaylist: Playlist> {
    #[serde(default = "Vec::new")]
    pub playlist: Vec<TPlaylist>
}

/// internetRadioStations.
/// # Example
/// ```
/// # use subsonic::models::InternetRadioStations;
/// # use subsonic::opensubsonic::models::InternetRadioStation;
/// # serde_json::from_str::<InternetRadioStations<InternetRadioStation>>(r#"
/// {
///   "internetRadioStation": [
///     {
///       "id": "1",
///       "name": "HBR1.com - Dream Factory",
///       "streamUrl": "http://ubuntu.hbr1.com:19800/ambient.aac",
///       "homePageUrl": "http://www.hbr1.com/"
///     },
///     {
///       "id": "2",
///       "name": "HBR1.com - I.D.M. Tranceponder",
///       "streamUrl": "http://ubuntu.hbr1.com:19800/trance.ogg",
///       "homePageUrl": "http://www.hbr1.com/"
///     },
///     {
///       "id": "3",
///       "name": "4ZZZ Community Radio",
///       "streamUrl": "https://stream.4zzz.org.au:9200/4zzz",
///       "homePageUrl": "https://4zzzfm.org.au"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InternetRadioStations<TInternetRadioStation: InternetRadioStation> {
    /// A list of internetRadioStation
    #[serde(default = "Vec::new")]
    pub internet_radio_station: Vec<TInternetRadioStation>,
}

/// A bookmark.
/// # Example
/// ```
/// # use subsonic::models::Bookmark;
/// # use subsonic::opensubsonic::models::Child;
/// # serde_json::from_str::<Bookmark<Child>>(r#"
/// {
///   "entry": {
///     "id": "113bf5989ad15ce2cf1834ba9622983f",
///     "parent": "b87a936c682c49d4494c7ccb08c22d23",
///     "isDir": false,
///     "title": "Stay Out Here",
///     "album": "Shaking The Habitual",
///     "artist": "The Knife",
///     "track": 11,
///     "year": 2013,
///     "genre": "Electronic",
///     "coverArt": "al-b87a936c682c49d4494c7ccb08c22d23_0",
///     "size": 21096309,
///     "contentType": "audio/mp4",
///     "suffix": "m4a",
///     "duration": 642,
///     "bitRate": 257,
///     "bitDepth": 16,
///     "samplingRate": 44100,
///     "channelCount": 2,
///     "path": "The Knife/Shaking The Habitual/11 - Stay Out Here.m4a",
///     "created": "2023-03-13T16:30:35Z",
///     "albumId": "b87a936c682c49d4494c7ccb08c22d23",
///     "artistId": "b29e9a9d780cb0e133f3add5662771b9",
///     "type": "music",
///     "isVideo": false,
///     "bookmarkPosition": 129000
///   },
///   "position": 129000,
///   "username": "demo",
///   "comment": "",
///   "created": "2023-03-13T16:30:35Z",
///   "changed": "2023-03-13T16:30:35Z"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark<TChild: Child> {
    /// Bookmark position in milliseconds
    pub position: u64,
    /// Username
    pub username: Box<str>,
    /// Bookmark comment
    pub comment: Option<Box<str>>,
    /// Bookmark creation date \[ISO 8601\]
    pub created: Box<str>,
    /// Bookmark last updated date \[ISO 8601\]
    pub changed: Box<str>,
    /// The bookmark file
    pub entry: TChild
}

/// Bookmarks list.
/// # Example
/// ```
/// # use subsonic::models::Bookmarks;
/// # use subsonic::opensubsonic::models::Child;
/// # serde_json::from_str::<Bookmarks<Child>>(r#"
/// {
///   "bookmark": [
///     {
///       "entry": {
///         "id": "113bf5989ad15ce2cf1834ba9622983f",
///         "parent": "b87a936c682c49d4494c7ccb08c22d23",
///         "isDir": false,
///         "title": "Stay Out Here",
///         "album": "Shaking The Habitual",
///         "artist": "The Knife",
///         "track": 11,
///         "year": 2013,
///         "genre": "Electronic",
///         "coverArt": "al-b87a936c682c49d4494c7ccb08c22d23_0",
///         "size": 21096309,
///         "contentType": "audio/mp4",
///         "suffix": "m4a",
///         "duration": 642,
///         "bitRate": 257,
///         "bitDepth": 16,
///         "samplingRate": 44100,
///         "channelCount": 2,
///         "path": "The Knife/Shaking The Habitual/11 - Stay Out Here.m4a",
///         "created": "2023-03-13T16:30:35Z",
///         "albumId": "b87a936c682c49d4494c7ccb08c22d23",
///         "artistId": "b29e9a9d780cb0e133f3add5662771b9",
///         "type": "music",
///         "isVideo": false,
///         "bookmarkPosition": 129000
///       },
///       "position": 129000,
///       "username": "demo",
///       "comment": "",
///       "created": "2023-03-13T16:30:35Z",
///       "changed": "2023-03-13T16:30:35Z"
///     },
///     {
///       "entry": {
///         "id": "2b42782333450d02b177823e729664af",
///         "parent": "dc8d8889a6fe08d8da7698c7ee1de61c",
///         "isDir": false,
///         "title": "Ill with the Skills",
///         "album": "First Words",
///         "artist": "The Polish Ambassador",
///         "track": 17,
///         "year": 2014,
///         "coverArt": "mf-2b42782333450d02b177823e729664af_641edeb3",
///         "size": 6219581,
///         "contentType": "audio/mpeg",
///         "suffix": "mp3",
///         "duration": 255,
///         "bitRate": 194,
///         "bitDepth": 16,
///         "samplingRate": 44100,
///         "channelCount": 2,
///         "path": "The Polish Ambassador/First Words/17 - Ill with the Skills.mp3",
///         "playCount": 1,
///         "played": "2023-03-15T15:23:37Z",
///         "created": "2023-03-25T11:44:51Z",
///         "albumId": "dc8d8889a6fe08d8da7698c7ee1de61c",
///         "artistId": "64e1f796b283545d329cdf6a31a31dbe",
///         "type": "music",
///         "isVideo": false,
///         "bookmarkPosition": 7000
///       },
///       "position": 7000,
///       "username": "demo",
///       "comment": "playSub bookmark",
///       "created": "2023-03-25T11:44:51Z",
///       "changed": "2023-03-25T11:44:51Z"
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bookmarks<TChild: Child> {
    /// List of bookmark
    #[serde(default = "Vec::new")]
    pub bookmark: Vec<Bookmark<TChild>>,
}

/// searchResult
/// # Example
/// ```
/// # use subsonic::models::SearchResult;
/// # use subsonic::opensubsonic::models::Child;
/// # serde_json::from_str::<SearchResult<Child>>(r#"
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
pub struct SearchResult<TChild: Child> {
    pub offset: u32,
    pub total_hits: u32,
    #[serde(default = "Vec::new", rename = "match")]
    pub match_: Vec<TChild>,
}

/// searchResult2
/// # Example
/// ```
/// # use subsonic::models::SearchResult2;
/// # use subsonic::opensubsonic::models::Child;
/// # serde_json::from_str::<SearchResult2<Child>>(r#"
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
pub struct SearchResult2<TChild: Child> {
    /// Starred artists
    #[serde(default)]
    pub artist: Vec<Artist>,
    /// Starred albums
    #[serde(default = "Vec::new")]
    pub album: Vec<TChild>,
    /// Starred songs
    #[serde(default = "Vec::new")]
    pub song: Vec<TChild>,
}

/// search3 Result.
/// # Example
/// ```
/// # use subsonic::models::SearchResult3;
/// # use subsonic::opensubsonic::models::Child;
/// # serde_json::from_str::<SearchResult3<Child>>(r#"
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
pub struct SearchResult3<TChild: Child> {
    /// Matching artists
    #[serde(default)]
    pub artist: Vec<ArtistID3>,
    /// Matching albums
    #[serde(default)]
    pub album: Vec<AlbumID3>,
    /// Matching songs
    #[serde(default = "Vec::new")]
    pub song: Vec<TChild>,
}

/// An artist from ID3 tags.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::ArtistID3>(r#"
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
/// # use subsonic::models::AlbumID3;
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
