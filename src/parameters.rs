use serde::Serialize;

/// Parameters for the [`crate::Client<T>::search3()`] method
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Search3Parameters {
    /// Search query.
    pub query: Box<str>,
    /// Maximum number of artists to return.
    pub artist_count: u32,
    /// Search result offset for artists. Used for paging.
    pub artist_offset: u32,
    /// Maximum number of albums to return.
    pub album_count: u32,
    /// Search result offset for albums. Used for paging
    pub album_offset: u32,
    /// Maximum number of songs to return.
    pub song_count: u32,
    /// Search result offset for songs. Used for paging.
    pub song_offset: u32,
    /// (Since Subsonic 1.12.0) Only return results from music folder with the given ID. See
    /// [`crate::Client<T>::getMusicFolders()`].
    pub music_folder_id: Option<Box<str>>,
}

impl Search3Parameters {
    /// Query artists, albums and songs based on the `query` Parameter
    pub fn query(query: impl Into<Box<str>>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }
    /// Query exclusively artists
    pub fn artist(query: impl Into<Box<str>>, artist_count: u32) -> Self {
        Self {
            query: query.into(),
            artist_count,
            album_count: 0,
            song_count: 0,
            ..Default::default()
        }
    }
    /// Query exclusively artists with an offset
    pub fn artist_with_offset(
        query: impl Into<Box<str>>,
        artist_count: u32,
        artist_offset: u32,
    ) -> Self {
        Self {
            query: query.into(),
            artist_count,
            artist_offset,
            album_count: 0,
            song_count: 0,
            ..Default::default()
        }
    }
    /// Query exclusively albums
    pub fn album(query: impl Into<Box<str>>, album_count: u32) -> Self {
        Self {
            query: query.into(),
            artist_count: 0,
            album_count,
            song_count: 0,
            ..Default::default()
        }
    }
    /// Query exclusively albums with an offset
    pub fn album_with_offset(
        query: impl Into<Box<str>>,
        album_count: u32,
        album_offset: u32,
    ) -> Self {
        Self {
            query: query.into(),
            artist_count: 0,
            album_count,
            album_offset,
            song_count: 0,
            ..Default::default()
        }
    }
    /// Query exclusively songs
    pub fn song(query: impl Into<Box<str>>, song_count: u32) -> Self {
        Self {
            query: query.into(),
            artist_count: 0,
            album_count: 0,
            song_count,
            ..Default::default()
        }
    }
    /// Query exclusively songs with offset
    pub fn song_with_offset(query: impl Into<Box<str>>, song_count: u32, song_offset: u32) -> Self {
        Self {
            query: query.into(),
            artist_count: 0,
            album_count: 0,
            song_count,
            song_offset,
            ..Default::default()
        }
    }
    /// Query artists, albums and songs
    pub fn all(
        query: impl Into<Box<str>>,
        artist_count: u32,
        album_count: u32,
        song_count: u32,
    ) -> Self {
        Self {
            query: query.into(),
            artist_count,
            album_count,
            song_count,
            ..Default::default()
        }
    }
    /// Query artists, albums and songs with offset
    pub fn all_with_offset(
        query: impl Into<Box<str>>,
        artist_count: u32,
        artist_offset: u32,
        album_count: u32,
        album_offset: u32,
        song_count: u32,
        song_offset: u32,
    ) -> Self {
        Self {
            query: query.into(),
            artist_count,
            artist_offset,
            album_count,
            album_offset,
            song_count,
            song_offset,
            ..Default::default()
        }
    }
}

impl Default for Search3Parameters {
    /// This is documentation on a method **inside** a Trait implementation
    fn default() -> Self {
        Self {
            query: "".into(),
            artist_count: 20,
            artist_offset: 0,
            album_count: 20,
            album_offset: 0,
            song_count: 20,
            song_offset: 0,
            music_folder_id: None,
        }
    }
}

/// Parameters for the [`crate::Client<T>::star()`] and [`crate::Client<T>::unstar()`] methods
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StarParameters {
    /// The ID of the file (song) or folder (album/artist) to star/unstar. Multiple parameters allowed.
    pub id: Option<Box<str>>,
    /// The ID of an album to star/unstar. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
    pub album_id: Option<Box<str>>,
    /// The ID of an artist to star/unstar. Use this rather than `id` if the client accesses the media collection according to ID3 tags rather than file structure. Multiple parameters allowed.
    pub artist_id: Option<Box<str>>,
}

impl StarParameters {
    pub fn new(
        id: Option<impl Into<Box<str>>>,
        album_id: Option<impl Into<Box<str>>>,
        artist_id: Option<impl Into<Box<str>>>,
    ) -> Self {
        Self {
            id: id.map(Into::into),
            album_id: album_id.map(Into::into),
            artist_id: artist_id.map(Into::into),
        }
    }
    /// Star/Unstar based on `id` parameter only
    pub fn id(id: impl Into<Box<str>>) -> Self {
        Self {
            id: Some(id.into()),
            ..Default::default()
        }
    }
    /// Star/Unstar based on `album_id` parameter only
    pub fn album_id(album_id: impl Into<Box<str>>) -> Self {
        Self {
            album_id: Some(album_id.into()),
            ..Default::default()
        }
    }
    /// Star/Unstar based on `artist_id` parameter only
    pub fn artist_id(artist_id: impl Into<Box<str>>) -> Self {
        Self {
            artist_id: Some(artist_id.into()),
            ..Default::default()
        }
    }
    /// Star/Unstar based on all three parameters at once
    pub fn all(
        id: impl Into<Box<str>>,
        album_id: impl Into<Box<str>>,
        artist_id: impl Into<Box<str>>,
    ) -> Self {
        Self {
            id: Some(id.into()),
            album_id: Some(album_id.into()),
            artist_id: Some(artist_id.into()),
        }
    }
}

/// Parameters for the [`crate::Client<T>::get_lyrics()`] method
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLyricsParameters {
    /// The artist name.
    pub artist: Option<Box<str>>,
    /// The song title.
    pub title: Option<Box<str>>,
}

impl GetLyricsParameters {
    /// Fetch lyrics solely based on title
    pub fn title(title: impl Into<Box<str>>) -> Self {
        Self {
            title: Some(title.into()),
            ..Default::default()
        }
    }
    /// Fetch lyrics solely based on artist
    pub fn artist(artist: impl Into<Box<str>>) -> Self {
        Self {
            artist: Some(artist.into()),
            ..Default::default()
        }
    }
    /// Fetch lyrics based on artist and title
    pub fn all(artist: impl Into<Box<str>>, title: impl Into<Box<str>>) -> Self {
        Self {
            artist: Some(artist.into()),
            title: Some(title.into()),
        }
    }
}
