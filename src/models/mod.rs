mod subsonic_response;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub use subsonic_response::*;

#[derive(Serialize, Deserialize)]
pub struct SubsonicError {
    pub code: u8,
    pub message: String,
}

/// Subsonic Response without any additional information to be wrapped
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::SubsonicBasicResponse>(r#"
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
/// # serde_json::from_str::<subsonic::models::SubsonicBasicData>(r#"
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

/// OpenSubsonic Response without any additional information to be wrapped
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::OpenSubsonicBasicResponse>(r#"
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
/// # serde_json::from_str::<subsonic::models::OpenSubsonicBasicData>(r#"
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
/// # serde_json::from_str::<subsonic::models::OpenSubsonicExtension>(r#"
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

/// A word or syllable cue within a cueLine.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Cue>(r#"
/// {
///     "start": 2747,
///     "end": 3018,
///     "value": "눈",
///     "byteStart": 0,
///     "byteEnd": 2
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
    pub value: Box<str>,
}

/// Word/syllable-level timing data for a lyrics line or agent layer.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::CueLine>(r#"
/// {
///   "index": 0,
///   "start": 2747,
///   "end": 6214,
///   "value": "눈을 뜬 순간",
///   "cue": [
///     { "start": 2747, "end": 3018, "value": "눈", "byteStart": 0, "byteEnd": 2 },
///     { "start": 3018, "end": 3179, "value": "을", "byteStart": 3, "byteEnd": 5 },
///     { "start": 3179, "end": 3582, "value": " ", "byteStart": 6, "byteEnd": 6 },
///     { "start": 3582, "end": 4100, "value": "뜬", "byteStart": 7, "byteEnd": 9 },
///     { "start": 4100, "end": 4500, "value": " ", "byteStart": 10, "byteEnd": 10 },
///     { "start": 4500, "end": 5200, "value": "순", "byteStart": 11, "byteEnd": 13 },
///     { "start": 5200, "end": 6214, "value": "간", "byteStart": 14, "byteEnd": 16 }
///   ]
/// }
/// # "#).unwrap();
/// ```
/// # Example with agent attribution
/// ```
/// # serde_json::from_str::<subsonic::models::CueLine>(r#"
/// {
///   "index": 0,
///   "start": 1000,
///   "end": 4000,
///   "value": "You and I",
///   "agentId": "lead",
///   "cue": [
///     { "start": 1000, "end": 1800, "value": "You ", "byteStart": 0, "byteEnd": 3 },
///     { "start": 1800, "end": 2400, "value": "and ", "byteStart": 4, "byteEnd": 7 },
///     { "start": 2400, "end": 3200, "value": "I", "byteStart": 8, "byteEnd": 8 }
///   ]
/// }
/// # "#).unwrap();
/// ```
/// # Example with amibguous repeated text
/// ```
/// # serde_json::from_str::<subsonic::models::CueLine>(r#"
/// {
///   "index": 0,
///   "start": 0,
///   "end": 2400,
///   "value": "Oh love love me tonight",
///   "cue": [
///     { "start": 0, "end": 300, "value": "Oh", "byteStart": 0, "byteEnd": 1 },
///     { "start": 900, "end": 1300, "value": "love", "byteStart": 8, "byteEnd": 11 },
///     { "start": 1300, "end": 1600, "value": "me", "byteStart": 13, "byteEnd": 14 },
///     { "start": 1600, "end": 2400, "value": "tonight", "byteStart": 16, "byteEnd": 22 }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AgentRole {
    /// Lead/default vocal layer
    Main,
    /// Additional explicit individual voice part
    Voice,
    /// Background vocals
    Bg,
    /// Group/chorus vocals
    Group,
}

/// Reusable metadata for a vocal agent within a structuredLyrics entry.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Agent>(r#"
/// {
///   "id": "lead",
///   "role": "main",
///   "name": "Chris Martin"
/// }
/// # "#).unwrap();
/// ```
/// # Example agent list within one `structuredLyrics` entry
/// ```
/// # serde_json::from_str::<Vec<subsonic::models::Agent>>(r#"
/// [
///   { "id": "lead", "role": "main", "name": "Chris Martin" },
///   { "id": "guest", "role": "voice", "name": "Jin" },
///   { "id": "choir", "role": "group", "name": "All" },
///   { "id": "backing", "role": "bg" }
/// ]
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Line>(r#"
/// {
///   "start": 0,
///   "value": "It's bugging me"
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// The start time of the lyrics, relative to the start time of the track, in milliseconds. If this is not part of synced lyrics, start **must** be omitted
    #[serde(default)]
    pub start: u32,
    /// The actual text of this line
    pub value: Box<str>,
}

/// The primary lyric-layer classification for a `structuredLyrics` entry.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StructuredLyricsKind {
    /// Primary vocals for this entry, default if omitted
    Main,
    /// A translation of another lyric layer into another language
    Translation,
    /// A phonetic/romanized rendering, e.g. romaji for Japanese, pinyin for Chinese
    Pronunciation,
}

/// Structured lyrics.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::EnhancedStructuredLyrics>(r#"
/// {
///   "kind": "main",
///   "lang": "ko",
///   "synced": true,
///   "line": [
///     { "start": 2747, "value": "눈을 뜬 순간" },
///     { "start": 6214, "value": "모든 게 달라졌어" }
///   ],
///   "cueLine": [
///     {
///       "index": 0,
///       "start": 2747,
///       "end": 6214,
///       "value": "눈을 뜬 순간",
///       "cue": [
///         { "start": 2747, "end": 3018, "value": "눈", "byteStart": 0, "byteEnd": 2 },
///         { "start": 3018, "end": 3179, "value": "을", "byteStart": 3, "byteEnd": 5 },
///         { "start": 3179, "end": 3582, "value": " ", "byteStart": 6, "byteEnd": 6 },
///         { "start": 3582, "end": 4100, "value": "뜬", "byteStart": 7, "byteEnd": 9 },
///         { "start": 4100, "end": 4500, "value": " ", "byteStart": 10, "byteEnd": 10 },
///         { "start": 4500, "end": 5200, "value": "순", "byteStart": 11, "byteEnd": 13 },
///         { "start": 5200, "end": 6214, "value": "간", "byteStart": 14, "byteEnd": 16 }
///       ]
///     },
///     {
///       "index": 1,
///       "start": 6214,
///       "end": 9000,
///       "value": "모든 게 달라졌어",
///       "cue": [
///         { "start": 6214, "end": 6800, "value": "모", "byteStart": 0, "byteEnd": 2 },
///         { "start": 6800, "end": 7200, "value": "든", "byteStart": 3, "byteEnd": 5 },
///         { "start": 7200, "end": 7600, "value": " ", "byteStart": 6, "byteEnd": 6 },
///         { "start": 7600, "end": 8000, "value": "게", "byteStart": 7, "byteEnd": 9 },
///         { "start": 8000, "end": 8400, "value": " ", "byteStart": 10, "byteEnd": 10 },
///         { "start": 8400, "end": 9000, "value": "달라졌어", "byteStart": 11, "byteEnd": 22 }
///       ]
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
/// # Example with agent attribution
/// ```
/// # serde_json::from_str::<subsonic::models::EnhancedStructuredLyrics>(r#"
/// {
///   "kind": "main",
///   "lang": "eng",
///   "synced": true,
///   "line": [
///     { "start": 1000, "value": "You and I" },
///     { "start": 4000, "value": "Under this sky" },
///     { "start": 7000, "value": "Together tonight" }
///   ],
///   "agents": [
///     { "id": "lead", "role": "main", "name": "Chris Martin" },
///     { "id": "guest", "role": "voice", "name": "Jin" },
///     { "id": "choir", "role": "group", "name": "All" }
///   ],
///   "cueLine": [
///     {
///       "index": 0,
///       "agentId": "lead",
///       "start": 1000,
///       "end": 4000,
///       "value": "You and I",
///       "cue": [
///         { "start": 1000, "end": 1800, "value": "You ", "byteStart": 0, "byteEnd": 3 },
///         { "start": 1800, "end": 2400, "value": "and ", "byteStart": 4, "byteEnd": 7 },
///         { "start": 2400, "end": 3200, "value": "I", "byteStart": 8, "byteEnd": 8 }
///       ]
///     },
///     {
///       "index": 1,
///       "agentId": "guest",
///       "start": 4000,
///       "end": 7000,
///       "value": "Under this sky",
///       "cue": [
///         { "start": 4000, "end": 4800, "value": "Un", "byteStart": 0, "byteEnd": 1 },
///         { "start": 4800, "end": 5400, "value": "der ", "byteStart": 2, "byteEnd": 5 },
///         { "start": 5400, "end": 5900, "value": "this ", "byteStart": 6, "byteEnd": 10 },
///         { "start": 5900, "end": 7000, "value": "sky", "byteStart": 11, "byteEnd": 13 }
///       ]
///     },
///     {
///       "index": 2,
///       "agentId": "choir",
///       "start": 7000,
///       "end": 10000,
///       "value": "Together tonight",
///       "cue": [
///         { "start": 7000, "end": 8000, "value": "To", "byteStart": 0, "byteEnd": 1 },
///         { "start": 8000, "end": 8800, "value": "ge", "byteStart": 2, "byteEnd": 3 },
///         { "start": 8800, "end": 9200, "value": "ther ", "byteStart": 4, "byteEnd": 8 },
///         { "start": 9200, "end": 10000, "value": "tonight", "byteStart": 9, "byteEnd": 15 }
///       ]
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
    #[serde(default)]
    pub cue_line: Vec<CueLine>,
}

/// List of structured lyrics.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::EnhancedLyricsList>(r#"
/// {
///   "structuredLyrics": [
///     {
///       "displayArtist": "Muse",
///       "displayTitle": "Hysteria",
///       "lang": "eng",
///       "offset": -100,
///       "synced": true,
///       "line": [
///         {
///           "start": 0,
///           "value": "It's bugging me"
///         },
///         {
///           "start": 2000,
///           "value": "Grating me"
///         },
///         {
///           "start": 3001,
///           "value": "And twisting me around..."
///         }
///       ]
///     },
///     {
///       "displayArtist": "Muse",
///       "displayTitle": "Hysteria",
///       "lang": "xxx",
///       "offset": 100,
///       "synced": false,
///       "line": [
///         {
///           "value": "It's bugging me"
///         },
///         {
///           "value": "Grating me"
///         },
///         {
///           "value": "And twisting me around..."
///         }
///       ]
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EnhancedLyricsList {
    /// Structured lyrics. There can be multiple lyrics of the same type with the same language
    #[serde(default)]
    pub structured_lyrics: Vec<EnhancedStructuredLyrics>,
}

/// Structured lyrics.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::StructuredLyrics>(r#"
/// {
///   "displayArtist": "Muse",
///   "displayTitle": "Hysteria",
///   "lang": "xxx",
///   "offset": -100,
///   "synced": true,
///   "line": [
///     {
///       "start": 0,
///       "value": "It's bugging me"
///     },
///     {
///       "start": 2000,
///       "value": "Grating me"
///     },
///     {
///       "start": 3001,
///       "value": "And twisting me around..."
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
    pub offset: i32,
    /// Reusable per-track attribution metadata for `cueLine` entries. When present, **must** contain at least one entry, and each `agents[].id` **must** be unique within this `structuredLyrics` entry. `agents` are optional for simple unattributed single-layer lyrics. When a `structuredLyrics` entry represents multiple vocal agents/layers, it **must** include `agents`; a single-agent attributed/default entry may also include `agents`, and if it does, exactly one agent **must** use `role: "main"`. `agents` should not be emitted without `cueLine` data
    #[serde(default)]
    pub agents: Vec<Agent>,
}

/// List of structured lyrics.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::LyricsList>(r#"
/// {
///   "structuredLyrics": [
///     {
///       "displayArtist": "Muse",
///       "displayTitle": "Hysteria",
///       "lang": "eng",
///       "offset": -100,
///       "synced": true,
///       "line": [
///         {
///           "start": 0,
///           "value": "It's bugging me"
///         },
///         {
///           "start": 2000,
///           "value": "Grating me"
///         },
///         {
///           "start": 3001,
///           "value": "And twisting me around..."
///         }
///       ]
///     },
///     {
///       "displayArtist": "Muse",
///       "displayTitle": "Hysteria",
///       "lang": "xxx",
///       "offset": 100,
///       "synced": false,
///       "line": [
///         {
///           "value": "It's bugging me"
///         },
///         {
///           "value": "Grating me"
///         },
///         {
///           "value": "And twisting me around..."
///         }
///       ]
///     }
///   ]
/// }
/// # "#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LyricsList {
    /// Structured lyrics. There can be multiple lyrics of the same type with the same language
    #[serde(default)]
    pub structured_lyrics: Vec<StructuredLyrics>,
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

/// search3 Result.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::SearchResult3>(r#"
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

/// A work associated with a song.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Work>(r#"
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
/// # serde_json::from_str::<subsonic::models::Movement>(r#"
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
/// # serde_json::from_str::<subsonic::models::ReplayGain>(r#"
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
/// # serde_json::from_str::<subsonic::models::Contributor>(r#"
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

// TODO split into SubsonicChild and OpenSubsonicChild
/// A media.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::Child>(r#"
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
    /// **OS** The bit depth of the media.
    pub bit_depth: Option<u8>,
    /// **OS** The sampling rate of the media.
    pub sampling_rate: Option<u32>,
    /// **OS** The number of channels of the media.
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
    /// **OS** The actual media type \[song/album/artist\] 
    /// Note: If you support `musicBrainzId` you must support this field to ensure clients knows what the ID refers to.
    pub media_type: Option<Box<str>>,
    /// The bookmark position in seconds
    pub bookmark_position: Option<u32>,
    /// The video original Width
    pub original_width: Option<u16>,
    /// The video original Height
    pub original_height: Option<u16>,
    /// **OS** Date the album was last played. \[ISO 8601\]
    pub played: Option<DateTime<Utc>>,
    /// **OS** The BPM of the song.
    pub bpm: Option<u16>,
    /// **OS** The comment tag of the song.
    pub comment: Option<Box<str>>,
    /// **OS** The song sort name.
    pub sort_name: Option<Box<str>>,
    /// **OS** The track MusicBrainzID.
    pub music_brainz_id: Option<Box<str>>,
    /// **OS** The track ISRC(s).
    pub isrc: Option<Vec<Box<str>>>,
    /// **OS** The list of all genres of the song.
    pub genres: Option<Vec<ItemGenre>>,
    /// **OS** The list of all song artists of the song. 
    /// (Note: Only the required [`ArtistID3`] fields should be returned by default)
    pub artists: Option<Vec<ArtistID3>>,
    /// **OS** The single value display artist.
    pub display_artist: Option<Box<str>>,
    /// **OS** The list of all album artists of the song. 
    /// (Note: Only the required [`ArtistID3`] fields should be returned by default)
    pub album_artists: Option<Vec<ArtistID3>>,
    /// **OS** The single value display album artist.
    pub display_album_artist: Option<Box<str>>,
    /// **OS** The list of all contributor artists of the song.
    pub contributors: Option<Vec<Contributor>>,
    /// **OS** The single value display composer.
    pub display_composer: Option<Box<str>>,
    /// **OS** The list of all moods of the song.
    pub moods: Option<Vec<Box<str>>>,
    /// **OS** The replaygain data of the song.
    pub replay_gain: Option<ReplayGain>,
    /// **OS** Returns “explicit”, “clean” or “”. 
    /// (For songs extracted from tags “ITUNESADVISORY”: 1 = explicit, 2 = clean, MP4 “rtng”: 1 or 4 = explicit, 2 = clean. See `albumID3` for albums)
    pub explicit_status: Option<Box<str>>,
    /// **OS** The list of works associated with the song.
    pub works: Option<Vec<Work>>,
    /// **OS** The list of movements associated with the song.
    pub movements: Option<Vec<Movement>>,
    /// **OS** The list of groupings associated with the song.
    pub groupings: Option<Vec<Box<str>>>
}

/// A disc title for an album, with an optional cover art.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::DiscTitle>(r#"
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

/// A date for a media item that may be just a year, or year-month, or full date.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::ItemDate>(r#"
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

/// A record label for an album.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::RecordLabel>(r#"
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

/// A genre in list of genres for an item
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::ItemGenre>(r#"
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

/// An album from ID3 tags.
/// # Example
/// ```
/// # serde_json::from_str::<subsonic::models::AlbumID3>(r#"
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
    // OpenSubsonic features
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
