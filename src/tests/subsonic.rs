use crate::*;

// Subsonic Demo
const SUBSONIC: super::SubsonicLogin = super::SubsonicLogin { 
    url: "http://demo.subsonic.org", 
    username: "guest4", 
    password: "guest" 
};

fn create_client() -> SubsonicClient {
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    SubsonicClient::new(SUBSONIC.url, parameters)
}

#[tokio::test]
async fn ping() {
    tests::ping(&create_client()).await;
}

#[tokio::test]
async fn get_license() {
    tests::get_license(&create_client()).await;
}

#[tokio::test]
async fn search() {
    let response = tests::search(&create_client(), SearchParameters::artist("PeerGynt Lobogris")).await;

    assert!(!response.match_.is_empty(), "{:#?}", response);
    assert_eq!(response.match_[0].artist.as_deref(), Some("PeerGynt Lobogris"), "{:#?}", response);
}

#[tokio::test]
async fn search2() {
    let term = "A Million Ways To Waste A Summer";
    let response = tests::search2(&create_client(), Search3Parameters::query(term)).await;

    assert_eq!(response.album.len(), 1, "{:#?}", response);
    assert_eq!(response.album[0].album.as_deref(), Some(term), "{:#?}", response);
}

#[tokio::test]
async fn search3() {
    let term = "A Million Ways To Waste A Summer";
    let response = tests::search3(&create_client(), Search3Parameters::query(term)).await;

    assert_eq!(response.album.len(), 1, "{:#?}", response);
    assert_eq!(response.album[0].name, term.into(), "{:#?}", response);
}

#[tokio::test]
async fn star_unstar_song() {
    tests::star_unstar_song_subsonic(&create_client()).await;
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
    // TODO Add working example
    // tests::get_lyrics(create_client(), GetLyricsParameters::all("", "")).await;
}

#[tokio::test]
async fn get_music_folders() {
    let music_folders = tests::get_music_folders(&create_client()).await;
    assert_eq!(music_folders.music_folder.len(), 1);
    
    let music_folder = music_folders.music_folder.first().unwrap();
    assert_eq!(music_folder.id, 0);
    assert_eq!(music_folder.name.as_deref(), "Music".into());
}

#[tokio::test]
async fn create_user() {
    let error_data = tests::create_user(&create_client()).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
}

#[tokio::test]
async fn delete_user() {
    let error_data = tests::delete_user(&create_client()).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
}

#[tokio::test]
async fn chat_message() {
    tests::chat_messages(&create_client()).await;
}

#[tokio::test]
async fn change_password() {
    let error_data = tests::change_password(&create_client(), SUBSONIC.username, SUBSONIC.password).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
}

#[tokio::test]
async fn bookmarks() {
    tests::bookmarks_subsonic(&create_client()).await;
}

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
    tests::subsonic_playlists(&create_client()).await;
}

#[tokio::test]
async fn podcasts() {
    tests::podcasts(&create_client()).await;
}

#[tokio::test]
async fn shares() {
    let result = tests::subsonic_shares(&create_client()).await;
    if let Err(SubsonicError::Failed(error)) = result {
        assert_eq!(error.code, SubsonicErrorCode::NotAuthorized);
    } else {
        panic!();
    }
}

#[tokio::test]
async fn download() {
    let client = create_client();
    tests::download_intentional_error(&client).await;
    tests::subsonic_download(&client).await;
}
