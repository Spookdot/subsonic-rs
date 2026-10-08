use crate::{opensubsonic::models::StructuredLyricsKind, *};

// Navidrome Demo (OpenSubsonic)
const NAVIDROME: super::SubsonicLogin = super::SubsonicLogin { 
    url: "https://demo.navidrome.org", 
    username: "demo", 
    password: "demo" 
};

fn create_client() -> OpenSubsonicClient {
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    OpenSubsonicClient::new(NAVIDROME.url, parameters)
}

#[tokio::test]
async fn ping() {
    tests::ping(&create_client()).await;
}

#[tokio::test]
async fn get_license() {
    tests::get_license(&create_client()).await;
}

// Skipped because Navidrome doesn't support this
#[tokio::test]
async fn search() {}

#[tokio::test]
async fn search2() {
    let term = "Pavel Tukki";
    let response = tests::search2(&create_client(), Search3Parameters::query(term)).await;
    
    assert_eq!(response.artist.len(), 1);
    assert_eq!(response.artist[0].name.as_ref(), term);
}

#[tokio::test]
async fn search3() {
    let term = "Pavel Tukki";
    let response = tests::search3(&create_client(), Search3Parameters::query(term)).await;
    
    assert_eq!(response.artist.len(), 1);
    assert_eq!(response.artist[0].name.as_ref(), term);
}

#[tokio::test]
async fn star_unstar_song() {
    tests::star_unstar_song_opensubsonic(&create_client()).await;
}

#[tokio::test]
async fn get_lyrics_title() {
    // TODO Add working example
    // tests::get_lyrics(create_client(), GetLyricsParameters::title("")).await;
}

#[tokio::test]
async fn get_lyrics_artist() {
    // TODO Add working example
    // tests::get_lyrics(create_client(), GetLyricsParameters::artist("")).await;
}

#[tokio::test]
async fn get_lyrics_both() {
    let response = tests::get_lyrics(&create_client(), GetLyricsParameters::all("Nine Inch Nails", "Letting You")).await;

    assert_eq!(response.title, "Letting You".into());
    assert_eq!(response.artist, "Nine Inch Nails".into());
    assert!(!response.value.is_empty());
}

#[tokio::test]
async fn get_lyrics_by_song_id() {
    let get_lyrics_response = tests::get_lyrics_by_song_id(&create_client(), "Letting You").await;
    let lyrics = get_lyrics_response.structured_lyrics[0].to_owned();

    assert_eq!(lyrics.display_title.as_deref(), Some("Letting You"), "Got {:?} instead of Letting You", lyrics.display_title);
    assert_eq!(lyrics.display_artist.as_deref(), Some("Nine Inch Nails"), "Got {:?} instead of Nine Inch Nails", lyrics.display_artist);
    assert_eq!(lyrics.line[0].value.as_ref(), "Letting You", "Got {:?} instead of the lyric \"Letting You\"", lyrics.line[0].value);
}

#[tokio::test]
async fn get_lyrics_by_song_id_enhanced() {
    let get_lyrics_response = tests::get_lyrics_by_song_id_enhanced(&create_client(), "Letting You").await;
    let lyrics = get_lyrics_response.structured_lyrics[0].to_owned();

    assert_eq!(lyrics.display_title.as_deref(), Some("Letting You"), "Got {:?} instead of Letting You", lyrics.display_title);
    assert_eq!(lyrics.display_artist.as_deref(), Some("Nine Inch Nails"), "Got {:?} instead of Nine Inch Nails", lyrics.display_artist);
    assert_eq!(lyrics.kind, Some(StructuredLyricsKind::Main));
    assert_eq!(lyrics.line[0].value.as_ref(), "Letting You", "Got {:?} instead of the lyric \"Letting You\"", lyrics.line[0].value);
}

#[tokio::test]
async fn get_opensubsonic_extension() {
    let extensions = tests::get_opensubsonic_extensions(&create_client()).await;
    // Filter for the songLyrics extension and error if it is missing
    let song_lyrics_extension = extensions.iter().find(|x| x.name.as_ref() == "songLyrics").unwrap();
    // Check that both versions of the songLyrics extension are supported
    assert_eq!(song_lyrics_extension.versions[0], 1, "SongLyrics Extension Version 1 should be supported");
    assert_eq!(song_lyrics_extension.versions[1], 2, "SongLyrics Extension Version 2 should be supported");
}

#[tokio::test]
async fn get_music_folders() {
    let music_folders = tests::get_music_folders(&create_client()).await;
    assert_eq!(music_folders.music_folder.len(), 1);
    
    let music_folder = music_folders.music_folder.first().unwrap();
    assert_eq!(music_folder.id, 1);
    assert_eq!(music_folder.name.as_deref(), "Music Library".into());
}

// Navidrome returns an error in the form of direct text instead of a proper JSON and therefore
// doesn't obey the Subsonic or OpenSubsonic API
/*
#[tokio::test]
async fn create_user() {}

#[tokio::test]
async fn delete_user() {}
*/

// TODO can't parse Navidrome response for some reason
/*
#[tokio::test]
async fn bookmarks() {
    tests::bookmarks_opensubsonic(&create_client()).await;
}
*/

#[tokio::test]
async fn create_internet_radio_stations() {
    let error_data = tests::create_internet_radio_stations(&create_client()).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
}

#[tokio::test]
async fn delete_internet_radio_stations() {
    let error_data = tests::delete_internet_radio_stations(&create_client()).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
}

#[tokio::test]
async fn get_internet_radio_stations() {
    let radio_stations = tests::get_internet_radio_stations(&create_client()).await;
    assert!(radio_stations.internet_radio_station.is_empty(), "{radio_stations:#?}");
}

#[tokio::test]
async fn playlists() {
    tests::opensubsonic_playlists(&create_client()).await;
}

// Currently not supported
/*
#[tokio::test]
async fn podcasts() {
    tests::podcasts(&create_client()).await;
}
*/

#[tokio::test]
async fn shares() {
    tests::opensubsonic_shares(&create_client()).await.unwrap();
}

#[tokio::test]
async fn download() {
    let client = create_client();
    tests::download_intentional_error(&client).await;
    tests::opensubsonic_download(&client).await;
}
