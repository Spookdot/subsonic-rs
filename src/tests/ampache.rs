use crate::*;

const AMPACHE: super::SubsonicLoginViaToken = super::SubsonicLoginViaToken {
    url: "https://demo.ampache.dev",
    token: "demodemo"
};

fn create_client() -> OpenSubsonicClient {
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    OpenSubsonicClient::new(AMPACHE.url, parameters)
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
    let response = tests::search(&create_client(), SearchParameters::artist("Crust")).await;

    assert!(!response.match_.is_empty(), "{:#?}", response);
    assert_eq!(response.match_[0].artist.as_deref(), Some("Crust"), "{:#?}", response);
}

#[tokio::test]
async fn search2() {
    let term = "Crust";
    let response = tests::search2(&create_client(), Search3Parameters::query(term)).await;
    
    assert_eq!(response.artist.len(), 1);
    assert_eq!(response.artist[0].name.as_ref(), term);
}

#[tokio::test]
async fn search3() {
    let term = "Crust";
    let response = tests::search3(&create_client(), Search3Parameters::query(term)).await;
    
    assert_eq!(response.artist.len(), 1);
    assert_eq!(response.artist[0].name.as_ref(), term);
}

#[tokio::test]
async fn star_unstar_song() {
    tests::star_unstar_song_opensubsonic(&create_client()).await;
}

// TODO Add working example
/*
#[tokio::test]
async fn get_lyrics_title() {
    // tests::get_lyrics(create_client(), GetLyricsParameters::title("")).await;
}
*/

// TODO Add working example
/*
#[tokio::test]
async fn get_lyrics_artist() {
    // tests::get_lyrics(create_client(), GetLyricsParameters::artist("")).await;
}
*/

// TODO Add working example
/*
#[tokio::test]
async fn get_lyrics_both() {
    // tests::get_lyrics(create_client(), GetLyricsParameters::all("", "")).await;
}
*/

// TODO
/*
#[tokio::test]
async fn get_lyrics_by_song_id() {}
*/

// TODO
/*
#[tokio::test]
async fn get_lyrics_by_song_id_enhanced() {}
*/

#[tokio::test]
async fn get_opensubsonic_extensions() {
    let extensions = tests::get_opensubsonic_extensions(&create_client()).await;
    assert_ne!(extensions.len(), 0, "No OpenSubsonic Extensions supported");
}

// Ampache skipped because it implements getMusicFolders incorrectly
// It returns a string for id when id should be an int
// See: https://opensubsonic.netlify.app/docs/responses/musicfolder/
/*
#[tokio::test]
async fn get_music_folders() {}
*/

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
    let message = "this is pretty cool";
    let subsonic_client = create_client();

    let add_chat_message_result = subsonic_client.add_chat_message(message).await;
    if let Err(SubsonicError::Failed(e)) = add_chat_message_result {
        assert_eq!(e.code, SubsonicErrorCode::GenericError);
    } else {
        panic!("Ampache should be returning an error object but either sent a working response or errored otherwise\n{:#?}", add_chat_message_result);
    }

    let messages = subsonic_client.get_chat_messages(None).await.unwrap();
    assert!(messages.chat_message.is_empty(), "Received Chat Messages even though the server doesn't support them\n{:#?}", messages);
}

// TODO is sending the wrong error back, figure out why
/*
#[tokio::test]
async fn change_password() {
    let result = tests::change_password(&create_client(), "demo", "demodemo").await;
    assert_eq!(result.code, SubsonicErrorCode::NotAuthorized);
}
*/

// TODO open issue, asks for required parameter despite giving all of them 
// https://demo.ampache.dev/rest/createBookmark?id=so-5&position=0&apiKey=demodemo&v=1.16.0&f=json&c=rust-subsonic-library
/*
#[tokio::test]
async fn bookmarks() {
    tests::bookmarks_opensubsonic(&create_client()).await;
}
*/

// TODO is sending the wrong error back, figure out why
/*
#[tokio::test]
async fn create_internet_radio_stations() {
    let error_data = tests::create_internet_radio_stations(&create_client()).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized, "{:#?}", error_data);
}
*/

// TODO is sending the wrong error back, figure out why
/*
#[tokio::test]
async fn delete_internet_radio_stations() {
    let error_data = tests::delete_internet_radio_stations(&create_client()).await;
    assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
}
*/

#[tokio::test]
async fn get_internet_radio_stations() {
    let radio_stations = tests::get_internet_radio_stations(&create_client()).await;
    assert_eq!(radio_stations.internet_radio_station[0].id, "li-5".into(), "{radio_stations:#?}");
}

#[tokio::test]
async fn playlists() {
    tests::opensubsonic_playlists(&create_client()).await;
}

#[tokio::test]
async fn podcasts() {
    tests::podcasts(&create_client()).await;
}

#[tokio::test]
async fn shares() {
    let result = tests::opensubsonic_shares(&create_client()).await;
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
    tests::opensubsonic_download(&client).await;
}
