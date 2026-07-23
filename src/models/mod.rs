mod subsonic_response;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
pub use subsonic_response::*;

#[derive(Serialize, Deserialize)]
pub struct SubsonicError {
    pub code: u8,
    pub message: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BasicResponse {
    pub subsonic_response: BasicData
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicData {
    pub status: Box<str>,
    pub version: Box<str>,
    pub open_subsonic: Option<bool>,
    pub server_version: Option<Box<str>>,
    #[serde(rename = "type")]
    pub type_: Option<Box<str>>,
}

/// A word or syllable cue within a cueLine.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase" )]
pub struct Cue {
    /// Start time in milliseconds
    pub start: u32,
    /// End time in milliseconds. Within a `cueLine`, `end` **must** be either present on **all** cues or **none**. 
    /// When the source provides partial end times, servers **must** fill missing values (e.g., using the next cue’s `start`, or the cueLine’s `end` for the final cue). 
    /// When no cues have end times (e.g., Enhanced LRC with start-only timing), `end` is omitted from all cues. 
    /// This is a documented contract rule; the OpenAPI schema does not enforce the all-or-none shape structurally
    #[serde(default)]
    pub end: u32,
    /// Zero-based inclusive UTF-8 byte offset into the parent `cueLine.value` where this cue begins
    pub byte_start: u32,
    /// Zero-based inclusive UTF-8 byte offset into the parent `cueLine.value` where this cue ends
    pub byte_end: u32,
    /// The text of this word or syllable
    pub value: Box<str>
}


/// Word/syllable-level timing data for a lyrics line or agent layer.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase" )]
pub struct CueLine {
    /// Zero-based index into the parent `line` array this cueLine corresponds to
    pub index: u32,
    /// Start time in milliseconds (may differ from the parent line if cues are more precise)
    #[serde(default)]
    pub start: u32,
    /// End time in milliseconds
    #[serde(default)]
    pub end: u32,
    /// Full text for this cueLine. 
    /// When agent attribution splits one parent line into multiple cueLines, this is the text for that cueLine’s agent/layer, not necessarily the parent line’s combined text. 
    /// Required because every nested `cue` defines `byteStart` / `byteEnd` against this exact final UTF-8 string
    pub value: Box<str>,
    /// Opaque identifier referencing an `agent` in the same `structuredLyrics` entry. 
    /// If the parent `structuredLyrics` entry includes `agents`, every cueLine in that entry **must** include `agentId`, and the value **must** match exactly one `agents[].id` in that entry. 
    /// If the parent entry does not include `agents`, cueLines **must not** include `agentId`. 
    /// When multiple cueLines share the same `index`, the cueLine whose referenced agent has `role: "main"` **must** come first
    pub argent_id: Option<Box<str>>,
    /// Ordered list of word/syllable cues. Every cue **must** include `byteStart` / `byteEnd` offsets into `value`
    pub cue: Vec<Cue>,
}

/// Semantic vocal-layer classification for cueLines that reference an agent.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase" )]
pub enum AgentRole {
    /// Lead/default vocal layer
    Main,
    /// Additional explicit individual voice part
    Voice,
    /// Background vocals
    Bg,
    /// Group/chorus vocals
    Group
}

/// Reusable metadata for a vocal agent within a structuredLyrics entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Agent {
    /// Opaque identifier for this agent. The value is only meaningful within the parent `structuredLyrics` entry and **must** be unique within that entry
    pub id: Box<str>,
    /// Semantic vocal-layer classification for cueLines that reference this agent. 
    /// One of: 
    /// `main` (lead/default vocal layer), 
    /// `voice` (additional explicit individual voice part), 
    /// `bg` (background vocals), 
    /// `group` (group/chorus vocals). 
    /// When a structuredLyrics entry uses agents for cue-attributed lyrics, it **must** define exactly one main agent
    pub role: AgentRole,
    /// Optional human-readable label for this agent, such as a singer or character name from the source metadata
    pub name: Option<Box<str>>,
}

/// One line of a song lyric.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Line {
    /// The start time of the lyrics, relative to the start time of the track, in milliseconds. If this is not part of synced lyrics, start **must** be omitted
    #[serde(default)]
    pub start: u32,
    /// The actual text of this line
    pub value: Box<str>,
}

/// The primary lyric-layer classification for a `structuredLyrics` entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase" )]
pub enum StructuredLyricsKind {
    /// Primary vocals for this entry, default if omitted
    Main, 
    /// A translation of another lyric layer into another language
    Translation, 
    /// A phonetic/romanized rendering, e.g. romaji for Japanese, pinyin for Chinese
    Pronunciation,
}

/// Structured lyrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnhancedStructuredLyrics {
    /// The lyrics language (ideally ISO 639). If the language is unknown (e.g. lrc file), the server **must** return `und` (ISO standard) or `xxx` (common value for taggers)
    pub lang: Box<str>,
    /// True if the lyrics are synced, false otherwise
    pub synced: bool,
    /// The actual lyrics. Ordered by start time (synced) or appearance order (unsynced)
    pub line: Vec<Line>,
    /// The artist name to display. This could be the localized name, or any other value
    pub display_artist: Option<Box<str>>,
    /// The title to display. This could be the song title (localized), or any other value
    pub display_title: Option<Box<str>>,
    /// The offset to apply to all lyrics, in milliseconds. Positive means lyrics appear sooner, negative means later. If not included, the offset **must** be assumed to be 0
    #[serde(default)]
    pub offset: i32,
    /// Reusable per-track attribution metadata for `cueLine` entries. 
    /// When present, **must** contain at least one entry, and each `agents[].id` **must** be unique within this `structuredLyrics` entry. 
    /// `agents` are optional for simple unattributed single-layer lyrics. 
    /// When a `structuredLyrics` entry represents multiple vocal agents/layers, it **must** include `agents`; 
    /// a single-agent attributed/default entry may also include `agents`, and if it does, exactly one agent **must** use `role: "main"`. 
    /// `agents` should not be emitted without `cueLine` data
    #[serde(default)]
    pub agents: Vec<Agent>,
    /// The primary lyric-layer classification for this `structuredLyrics` entry. 
    /// One of: 
    /// `main` (primary vocals for this entry, default if omitted), 
    /// `translation` (a translation of another lyric layer into another language), 
    /// `pronunciation` (a phonetic/romanized rendering, e.g. romaji for Japanese, pinyin for Chinese). 
    /// Tracks are independent across `kind` values; clients should not assume 1:1 line or cue alignment between entries. Only returned when `enhanced=true`. Added in `songLyrics` version 2
    pub kind: Option<StructuredLyricsKind>,
    /// Word/syllable-level timing data. 
    /// Each cueLine corresponds to a `line` by its `index` field. 
    /// Every cueLine **must** include `value`, and every nested cue **must** include `byteStart` / `byteEnd` offsets into that exact string. 
    /// If `agents` is present, every cueLine in the entry **must** include `agentId`; 
    /// if `agents` is absent, cueLines **must not** include `agentId`. 
    /// Only returned when `enhanced=true` and `synced` is `true`. 
    /// Added in `songLyrics` version 2
    pub cue_line: Vec<CueLine>,
}

/// List of structured lyrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnhancedLyricsList {
    /// Structured lyrics. There can be multiple lyrics of the same type with the same language
    #[serde(default)]
    pub structured_lyrics: Vec<EnhancedStructuredLyrics>,
}

/// Structured lyrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructuredLyrics {
    /// The lyrics language (ideally ISO 639). If the language is unknown (e.g. lrc file), the server **must** return `und` (ISO standard) or `xxx` (common value for taggers)
    pub lang: Box<str>,
    /// True if the lyrics are synced, false otherwise
    pub synced: bool,
    /// The actual lyrics. Ordered by start time (synced) or appearance order (unsynced)
    pub line: Vec<Line>,
    /// The artist name to display. This could be the localized name, or any other value
    pub display_artist: Option<Box<str>>,
    /// The title to display. This could be the song title (localized), or any other value
    pub display_title: Option<Box<str>>,
    /// The offset to apply to all lyrics, in milliseconds. Positive means lyrics appear sooner, negative means later. If not included, the offset **must** be assumed to be 0
    #[serde(default)]
    pub offset: u32,
    /// Reusable per-track attribution metadata for `cueLine` entries. When present, **must** contain at least one entry, and each `agents[].id` **must** be unique within this `structuredLyrics` entry. `agents` are optional for simple unattributed single-layer lyrics. When a `structuredLyrics` entry represents multiple vocal agents/layers, it **must** include `agents`; a single-agent attributed/default entry may also include `agents`, and if it does, exactly one agent **must** use `role: "main"`. `agents` should not be emitted without `cueLine` data
    #[serde(default)]
    pub agents: Vec<Agent>,
}

/// List of structured lyrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LyricsList {
    /// Structured lyrics. There can be multiple lyrics of the same type with the same language
    #[serde(default)]
    pub structured_lyrics: Vec<StructuredLyrics>,
}

/// Lyrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    /// The lyrics
    pub value: Box<str>,
    /// The artist name
    #[serde(default)]
    pub artist: Box<str>,
    /// The song title
    #[serde(default)]
    pub title: Box<str>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub valid: bool,
    #[serde(default)]
    pub email: Box<str>,
    #[serde(default)]
    pub license_expires: Box<str>,
    #[serde(default)]
    pub trial_expires: Box<str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchResult3 {
    #[serde(default)]
    pub artist: Vec<ArtistID3>,
    #[serde(default)]
    pub album: Vec<AlbumID3>,
    #[serde(default)]
    pub song: Vec<Song>,
}

/// A work associated with a song.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    /// The work name.
    name: String,
    /// The MusicBrainz Work ID.
    music_brainz_id: Option<String>
}

/// A movement associated with a song.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    /// The movement name.
    name: String,
    /// The movement number.
    number: Option<u16>,
    /// The total number of movements.
    count: Option<u16>
}

/// The replay gain data of a song.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// The contributor role.
    pub role: String,
    /// The subRole for roles that may require it.
    /// Ex: The instrument for the performer role (TMCL/performer tags).
    /// Note: For consistency between different tag formats,
    /// the TIPL sub roles should be directly exposed in the role field.
    pub sub_role: Option<String>,
    /// The artist taking on the role.
    /// (Note: Only the required [ArtistID3] fields should be returned by default)
    pub artist: ArtistID3
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    pub parent: Option<String>,
    pub is_dir: bool,
    pub title: String,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub track: Option<u16>,
    pub year: Option<u16>,
    pub genre: Option<String>,
    pub cover_art: Option<String>,
    pub size: Option<u32>,
    pub content_type: Option<String>,
    pub suffix: Option<String>,
    pub transcoded_content_type: Option<String>,
    pub transcoded_suffice: Option<String>,
    pub duration: Option<u32>,
    pub bit_rate: Option<u16>,
    pub bit_depth: Option<u8>,
    pub sampling_rate: Option<u32>,
    pub channel_count: Option<u8>,
    pub path: Option<String>,
    pub is_video: Option<bool>,
    pub user_rating: Option<u8>,
    pub average_rating: Option<f32>,
    pub play_count: Option<u32>,
    pub disc_number: Option<u8>,
    pub created: Option<DateTime<Utc>>,
    pub starred: Option<DateTime<Utc>>,
    pub album_id: Option<String>,
    pub artist_id: Option<String>,
    #[serde(rename = "type")]
    pub song_type: Option<String>,
    pub media_type: Option<String>,
    pub bookmark_position: Option<u32>,
    pub original_width: Option<u16>,
    pub original_height: Option<u16>,
    pub played: Option<DateTime<Utc>>,
    pub bpm: Option<u16>,
    pub comment: Option<String>,
    pub sort_name: Option<String>,
    pub music_brainz_id: Option<String>,
    pub isrc: Option<Vec<String>>,
    pub genres: Option<Vec<ItemGenre>>,
    pub artists: Option<Vec<ArtistID3>>,
    pub display_artist: Option<String>,
    pub album_artists: Option<Vec<ArtistID3>>,
    pub display_album_artist: Option<String>,
    pub contributors: Option<Vec<Contributor>>,
    pub display_composer: Option<String>,
    pub moods: Option<Vec<String>>,
    pub replay_gain: Option<ReplayGain>,
    pub explicit_status: Option<String>,
    pub works: Option<Vec<Work>>,
    pub movements: Option<Vec<Movement>>,
}

/// A disc title for an album, with an optional cover art.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscTitle {
    /// The disc number.
    disc: u16,
    /// The nname of the disc.
    title: String,
    /// The cover art ID of the disc.
    cover_art: Option<String>
}

/// A date for a media item that may be just a year, or year-month, or full date.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemDate {
    /// The year
    year: Option<u16>,
    /// The month (1-12)
    month: Option<u16>,
    /// The day (1-31)
    day: Option<u16>
}

/// A record label for an album.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecordLabel {
    /// The record label name.
    name: String
}

/// A genre in list of genres for an item
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemGenre {
    /// Genre name
    name: String
}

/// An album from ID3 tags.
/// Example:
/// ```json
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
///             "title": "Disc 0 title"
///             "coverArt": "42"
///         },
///         {
///             "disc": 2,
///             "title": "Disc 1 title"
///             "coverArt": "6547"
///         }
///     ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumID3 {
    /// The id of the album
    pub id: String,
    /// The album name.
    pub name: String,
    /// OpenSubSonic Feature
    /// The album version name (Remastered, Anniversary Box Set, ...).
    pub version: Option<String>,
    /// Artist name.
    pub artist: Option<String>,
    /// The id of the artist
    pub artist_id: Option<String>,
    /// A coverArt id.
    pub cover_art: Option<String>,
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
    pub genre: Option<String>,
    // OpenSubsonic features
    /// Date the album was last played. \[ISO 8601\]
    pub played: Option<String>,
    /// The user rating of the album. \[1-5\]
    pub user_rating: Option<u8>,
    /// The labels producing the album.
    pub record_labels: Option<Vec<RecordLabel>>,
    /// The album MusicBrainzID.
    pub music_brainz_id: Option<String>,
    /// The list of all genres of the album.
    pub genres: Option<Vec<ItemGenre>>,
    /// The list of all album artists of the album.
    /// (Note: Only the required [ArtistID3] fields should be returned by default)
    pub artists: Option<Vec<ArtistID3>>,
    /// The single value display artist
    pub display_artist: Option<String>,
    /// The types of this album release. (Album, Compilation, EP, Remix, ...)
    pub release_types: Option<Vec<String>>,
    /// The list of all moods of the album.
    pub moods: Option<Vec<String>>,
    /// The album sort name.
    pub sort_name: Option<String>,
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
    pub explicit_status: Option<String>,
    /// The list of all disc titles of the album.
    pub disc_titles: Option<Vec<DiscTitle>>,
}

/// An artist from ID3 tags.
/// Example:
/// ```json
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
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistID3 {
    /// The id of the artist.
    pub id: String,
    /// The artist name.
    pub name: String,
    /// A coverArt id.
    pub cover_art: Option<String>,
    /// An url to an external image source.
    pub artist_image_url: Option<String>,
    /// Artist album count.
    pub album_count: Option<u8>,
    /// Date the artist was starred. \[ISO 8601\]
    pub starred: Option<DateTime<Utc>>,
    // OpenSubsonic Features
    /// The artist MusicBrainzID.
    pub music_brainz_id: Option<String>,
    /// The artist sort name.
    pub sort_name: Option<String>,
    /// The list of all roles this artist has in the library.
    pub roles: Option<Vec<String>>,
}

