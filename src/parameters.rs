use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Search3Parameters {
    pub query: Box<str>,
    pub artist_count: u32,
    pub artist_offset: u32,
    pub album_count: u32,
    pub album_offset: u32,
    pub song_count: u32,
    pub song_offset: u32,
    pub music_folder_id: Option<Box<str>>,
}

impl Search3Parameters {
    pub fn query(query: impl Into<Box<str>>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }
}

impl Default for Search3Parameters {
    fn default() -> Self {
        Self {
            query: "".into(),
            artist_count: 20,
            artist_offset: 0,
            album_count: 20,
            album_offset: 0,
            song_count: 20,
            song_offset: 0,
            music_folder_id: None
        }
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StarParameters {
    pub id: Option<Box<str>>,
    pub album_id: Option<Box<str>>,
    pub artist_id: Option<Box<str>>,
}

impl StarParameters {
    // TODO consider changing all &str to `impl Into<Box<str>>`
    pub fn new(id: Option<impl Into<Box<str>>>, album_id: Option<impl Into<Box<str>>>, artist_id: Option<impl Into<Box<str>>>) -> Self {
        Self {
            id: id.map(Into::into),
            album_id: album_id.map(Into::into),
            artist_id: artist_id.map(Into::into),
        }
    }
    pub fn id(id: impl Into<Box<str>>) -> Self {
        Self { id: Some(id.into()), ..Default::default() }
    }
    pub fn album_id(album_id: impl Into<Box<str>>) -> Self {
        Self { album_id: Some(album_id.into()), ..Default::default() }
    }
    pub fn artist_id(artist_id: impl Into<Box<str>>) -> Self {
        Self { artist_id: Some(artist_id.into()), ..Default::default() }
    }
    pub fn all(
        id: impl Into<Box<str>>, 
        album_id: impl Into<Box<str>>, 
        artist_id: impl Into<Box<str>>
    ) -> Self {
        Self {
            id: Some(id.into()),
            album_id: Some(album_id.into()),
            artist_id: Some(artist_id.into()),
        }
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLyricsParameters {
    /// The artist name.
    pub artist: Option<Box<str>>,
    /// The song title.
    pub title: Option<Box<str>>,
}

impl GetLyricsParameters {
    pub fn new(artist: impl Into<Box<str>>, title: impl Into<Box<str>>) -> Self {
        Self {
            artist: Some(artist.into()),
            title: Some(title.into()),
        }
    }
    pub fn title(title: impl Into<Box<str>>) -> Self {
        Self { title: Some(title.into()), ..Default::default() }
    }
    pub fn artist(artist: impl Into<Box<str>>) -> Self {
        Self { artist: Some(artist.into()), ..Default::default() }
    }
}
