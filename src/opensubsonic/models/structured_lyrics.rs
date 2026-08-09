use serde::{Serialize, Deserialize};

/// A word or syllable cue within a cueLine.
/// # Example
/// ```
/// # use subsonic::opensubsonic::models::Cue;
/// # serde_json::from_str::<Cue>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::CueLine>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::CueLine>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::CueLine>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::Agent>(r#"
/// {
///   "id": "lead",
///   "role": "main",
///   "name": "Chris Martin"
/// }
/// # "#).unwrap();
/// ```
/// # Example agent list within one `structuredLyrics` entry
/// ```
/// # serde_json::from_str::<Vec<subsonic::opensubsonic::models::Agent>>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::Line>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::EnhancedStructuredLyrics>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::EnhancedStructuredLyrics>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::EnhancedLyricsList>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::StructuredLyrics>(r#"
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
/// # serde_json::from_str::<subsonic::opensubsonic::models::LyricsList>(r#"
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
