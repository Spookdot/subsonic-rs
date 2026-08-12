use super::opensubsonic::models::StructuredLyricsKind;
use super::*;

struct SubsonicLogin<'a> {
    url: &'a str,
    username: &'a str,
    password: &'a str,
}

struct SubsonicLoginViaToken<'a> {
    url: &'a str,
    token: &'a str,
}

// Navidrome Demo (OpenSubsonic)
const NAVIDROME: SubsonicLogin = SubsonicLogin { 
    url: "https://demo.navidrome.org", 
    username: "demo", 
    password: "demo" 
};

// Subsonic Demo
const SUBSONIC: SubsonicLogin = SubsonicLogin { 
    url: "http://demo.subsonic.org", 
    username: "guest4", 
    password: "guest" 
};

const AMPACHE: SubsonicLoginViaToken = SubsonicLoginViaToken {
    url: "https://demo.ampache.dev",
    token: "demodemo"
};

#[tokio::test]
async fn ping() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let ping_response_result = subsonic_client.ping().await;
    let ping_response = ping_response_result.unwrap();
    assert_eq!(ping_response.subsonic_response.status.as_ref(), "ok", "{:#?}", ping_response);

    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    let ping_response = subsonic_client.ping().await.unwrap();
    assert_eq!(ping_response.subsonic_response.status.as_ref(), "ok", "{:#?}", ping_response);

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let ping_response = subsonic_client.ping().await.unwrap();
    assert_eq!(ping_response.subsonic_response.status.as_ref(), "ok", "{:#?}", ping_response);
}

#[tokio::test]
async fn get_license() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let license = subsonic_client.get_license().await.unwrap();
    assert!(license.valid, "{:#?}", license);

    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    let license = subsonic_client.get_license().await.unwrap();
    assert!(license.valid, "{:#?}", license);

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let license = subsonic_client.get_license().await.unwrap();
    assert!(license.valid, "{:#?}", license);
}

#[tokio::test]
async fn search() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let search_response = subsonic_client.search(SearchParameters::artist("PeerGynt Lobogris")).await.unwrap();

    assert!(!search_response.match_.is_empty(), "{:#?}", search_response);
    assert_eq!(search_response.match_[0].artist.as_deref(), Some("PeerGynt Lobogris"), "{:#?}", search_response);
    // Navidrome doesn't support because deprecated

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let search_response = subsonic_client.search(SearchParameters::artist("Crust")).await.unwrap();
    
    assert!(!search_response.match_.is_empty(), "{:#?}", search_response);
    assert_eq!(search_response.match_[0].artist.as_deref(), Some("Crust"), "{:#?}", search_response);
}

#[tokio::test]
async fn search2() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let search2_response = subsonic_client.search2(Search3Parameters::query("A Million Ways To Waste A Summer")).await.unwrap();

    assert_eq!(search2_response.album.len(), 1, "{:#?}", search2_response);
    assert_eq!(search2_response.album[0].album.as_deref(), Some("A Million Ways To Waste A Summer"), "{:#?}", search2_response);
    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    let search2_response = subsonic_client.search2(Search3Parameters::query("Pavel Tukki")).await.unwrap();
    
    assert_eq!(search2_response.artist.len(), 1);
    assert_eq!(search2_response.artist[0].name.as_ref(), "Pavel Tukki");

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let search2_response = subsonic_client.search2(Search3Parameters::query("Crust")).await.unwrap();
    
    assert_eq!(search2_response.artist.len(), 1);
    assert_eq!(search2_response.artist[0].name.as_ref(), "Crust");
}

#[tokio::test]
async fn search3() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let search3_response = subsonic_client.search3(Search3Parameters::query("A Million Ways To Waste A Summer")).await.unwrap();

    assert_eq!(search3_response.album.len(), 1, "{:#?}", search3_response);
    assert_eq!(search3_response.album[0].name, "A Million Ways To Waste A Summer".into(), "{:#?}", search3_response);

    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    let search3_response = subsonic_client.search3(Search3Parameters::query("Pavel Tukki")).await.unwrap();
    
    assert_eq!(search3_response.artist.len(), 1);
    assert_eq!(search3_response.artist[0].name.as_ref(), "Pavel Tukki");

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let search3_response = subsonic_client.search3(Search3Parameters::query("Crust")).await.unwrap();
    
    assert_eq!(search3_response.artist.len(), 1);
    assert_eq!(search3_response.artist[0].name.as_ref(), "Crust");
}

#[tokio::test]
async fn star_unstar_song() {
    // Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let client = SubsonicClient::new(SUBSONIC.url, parameters);

    // Search Song
    let search3_response = client.search3(Search3Parameters::query("e")).await.unwrap();

    let song = search3_response.song[0].to_owned();
    let song_id = song.id;
    // Check if starred
    if song.starred.is_some() {
        // Unstar if starred
        let starred_response = client.unstar(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if unstarred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");

        // Star after unstarring
        let starred_response = client.star(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if starred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_some(), "Song should be starred after Star method was called");
    } else {
        // Star if unstarred
        let starred_response = client.star(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if starred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_some(), "Song should be starred after Star method was called");

        // Unstar after starring
        let starred_response = client.unstar(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if unstarred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");
    }

    // Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    // Search Song
    let search3_response = client.search3(Search3Parameters::query("")).await.unwrap();

    let song = search3_response.song[0].to_owned();
    let song_id = song.id;
    // Check if starred
    if song.starred.is_some() {
        // Unstar if starred
        let starred_response = client.unstar(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if unstarred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");

        // Star after unstarring
        let starred_response = client.star(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if starred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_some(), "Song should be starred after Star method was called");
    } else {
        // Star if unstarred
        let starred_response = client.star(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if starred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_some(), "Song should be starred after Star method was called");

        // Unstar after starring
        let starred_response = client.unstar(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if unstarred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");

    }

    // Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    // Search Song
    let search3_response = client.search3(Search3Parameters::query("")).await.unwrap();

    let song = search3_response.song[0].to_owned();
    let song_id = song.id;
    // Check if starred
    if song.starred.is_some() {
        // Unstar if starred
        let starred_response = client.unstar(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if unstarred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");

        // Star after unstarring
        let starred_response = client.star(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if starred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_some(), "Song should be starred after Star method was called");
    } else {
        // Star if unstarred
        let starred_response = client.star(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if starred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_some(), "Song should be starred after Star method was called");

        // Unstar after starring
        let starred_response = client.unstar(StarParameters::id(song_id.clone())).await.unwrap();
        assert_eq!(starred_response.subsonic_response.status.as_ref(), "ok");

        // Check if unstarred
        let song = client.get_song(&song_id).await.unwrap();
        assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");

    }
}

#[tokio::test]
async fn get_lyrics() {
    // TODO can't seem to get a response for any of these combinations
    // For Subsonic
    // let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    // let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    // Title Only
    // let get_lyrics_parameters = GetLyricsParameters::title("");
    // let get_lyrics_response = subsonic_client.get_lyrics(get_lyrics_parameters).await.unwrap();

    // Artist Only
    // let get_lyrics_parameters = GetLyricsParameters::artist("");
    // let get_lyrics_response = subsonic_client.get_lyrics(get_lyrics_parameters).await.unwrap();

    // Title and Artist
    // let get_lyrics_parameters = GetLyricsParameters::all("", "");
    // let get_lyrics_response = subsonic_client.get_lyrics(get_lyrics_parameters).await.unwrap();

    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    // TODO can't seem to get a response for title only and artist only
    // Title Only
    // let get_lyrics_parameters = GetLyricsParameters::title("");
    // let get_lyrics_response = subsonic_client.get_lyrics(get_lyrics_parameters).await.unwrap();

    // Artist Only
    // let get_lyrics_parameters = GetLyricsParameters::artist("");
    // let get_lyrics_response = subsonic_client.get_lyrics(get_lyrics_parameters).await.unwrap();

    // Title and Artist
    let get_lyrics_parameters = GetLyricsParameters::all("Nine Inch Nails", "Letting You");
    let get_lyrics_response = subsonic_client.get_lyrics(get_lyrics_parameters).await.unwrap();
    assert_eq!(get_lyrics_response.title, "Letting You".into(), "The titles don't match for Navidrome Title and Artist");
    assert_eq!(get_lyrics_response.artist, "Nine Inch Nails".into(), "The artists don't match for Navidrome Title and Artist");
    assert!(!get_lyrics_response.value.is_empty(), "Lyrics should not be empty for Navidrome Title and Artist");

    // TODO add case for Ampache
}

#[tokio::test]
async fn get_lyrics_by_song_id() {
    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    // Retrieve lyrics for song "Letting You" by Nine Inch Nails
    let get_lyrics_response = subsonic_client.get_lyrics_by_song_id("eMCrMHEMJG7IMu3soo0wsg").await.unwrap();
    let lyrics = get_lyrics_response.structured_lyrics[0].to_owned();

    assert_eq!(lyrics.display_title.as_deref(), Some("Letting You"), "Got {:?} instead of Letting You", &lyrics.display_title);
    assert_eq!(lyrics.display_artist.as_deref(), Some("Nine Inch Nails"), "Got {:?} instead of Nine Inch Nails", &lyrics.display_artist);
    assert_eq!(lyrics.line[0].value.as_ref(), "Letting You", "Got {:?} instead of the lyric \"Letting You\"", lyrics.line[0].value);

    // TODO add case for Ampache
}

#[tokio::test]
async fn get_lyrics_by_song_id_enhanced() {
    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    // Retrieve lyrics for song "Letting You" by Nine Inch Nails
    let get_lyrics_response = subsonic_client.get_lyrics_by_song_id_enhanced("eMCrMHEMJG7IMu3soo0wsg").await.unwrap();
    let lyrics = get_lyrics_response.structured_lyrics[0].to_owned();

    assert_eq!(lyrics.display_title.as_deref(), Some("Letting You"), "Got {:?} instead of Letting You", &lyrics.display_title);
    assert_eq!(lyrics.display_artist.as_deref(), Some("Nine Inch Nails"), "Got {:?} instead of Nine Inch Nails", &lyrics.display_artist);
    assert_eq!(lyrics.kind, Some(StructuredLyricsKind::Main));
    assert_eq!(lyrics.line[0].value.as_ref(), "Letting You", "Got {:?} instead of the lyric \"Letting You\"", lyrics.line[0].value);

    // TODO add case for Ampache
}

#[tokio::test]
async fn get_open_subsonic_extensions() {
    // Not supported by Subsonic
    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);

    let extensions = subsonic_client.get_open_subsonic_extensions().await.unwrap();
    // Filter for the songLyrics extension and error if it is missing
    let song_lyrics_extension = extensions.iter().find(|x| x.name.as_ref() == "songLyrics").unwrap();
    // Check that both versions of the songLyrics extension are supported
    assert_eq!(song_lyrics_extension.versions[0], 1, "SongLyrics Extension Version 1 should be supported");
    assert_eq!(song_lyrics_extension.versions[1], 2, "SongLyrics Extension Version 2 should be supported");

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let extensions = subsonic_client.get_open_subsonic_extensions().await.unwrap();
    assert_ne!(extensions.len(), 0, "No OpenSubsonic Extensions supported");
}

#[tokio::test]
async fn get_music_folders() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);
    
    let music_folders = subsonic_client.get_music_folders().await.unwrap();
    assert_eq!(music_folders.music_folder.len(), 1);
    
    let music_folder = music_folders.music_folder.first().unwrap();
    assert_eq!(music_folder.id, 0);
    assert_eq!(music_folder.name.as_deref(), "Music".into());

    // For Navidrome
    let parameters = SubsonicParameters::hashed_password("subsonic rust", NAVIDROME.username, NAVIDROME.password, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(NAVIDROME.url, parameters);
    
    let music_folders = subsonic_client.get_music_folders().await.unwrap();
    assert_eq!(music_folders.music_folder.len(), 1);
    
    let music_folder = music_folders.music_folder.first().unwrap();
    assert_eq!(music_folder.id, 1);
    assert_eq!(music_folder.name.as_deref(), "Music Library".into());

    // Ampache skipped because it implements getMusicFolders incorrectly
    // It returns a string for id when id should be an int
    // See: https://opensubsonic.netlify.app/docs/responses/musicfolder/
}

#[tokio::test]
async fn create_user() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let create_user_parameters = CreateUserParameters::with_default_roles("test", "test", "test");
    let create_user_result = subsonic_client.create_user(create_user_parameters).await;

    if let Err(SubsonicError::Failed(error_data)) = create_user_result {
        assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
        assert_eq!(error_data.message.as_ref(), "guest4 is not authorized to create new users.");
    } else {
        panic!("createUser endpoint for Subsonic should be returning an error, but isn't\n{create_user_result:#?}");
    }

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let create_user_parameters = CreateUserParameters::with_default_roles("test", "test", "test");
    let create_user_result = subsonic_client.create_user(create_user_parameters).await;

    if let Err(SubsonicError::Failed(error_data)) = create_user_result {
        assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
        assert_eq!(error_data.message.as_ref(), "User is not authorized for the given operation.");
        assert_eq!(error_data.help_url.as_deref(), Some("https://ampache.org/api/subsonic"));
    } else {
        panic!("createUser endpoint for Ampache should be returning an error, but isn't\n{create_user_result:#?}");
    }

    // Navidrome returns an error in the form of direct test instead of a proper JSON and therefore
    // doesn't obey the Subsonic or OpenSubsonic API
}

#[tokio::test]
async fn delete_user() {
    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    let delete_user_result = subsonic_client.delete_user("test").await;

    if let Err(SubsonicError::Failed(error_data)) = delete_user_result {
        assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
        assert_eq!(error_data.message.as_ref(), "guest4 is not authorized to delete users.");
    } else {
        panic!("deleteUser endpoint for Subsonic should be returning an error, but isn't\n{delete_user_result:#?}");
    }

    // For Ampache
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let delete_user_result = subsonic_client.delete_user("test").await;

    if let Err(SubsonicError::Failed(error_data)) = delete_user_result {
        assert_eq!(error_data.code, SubsonicErrorCode::NotAuthorized);
        assert_eq!(error_data.message.as_ref(), "User is not authorized for the given operation.");
        assert_eq!(error_data.help_url.as_deref(), Some("https://ampache.org/api/subsonic"));
    } else {
        panic!("deleteUser endpoint for Ampache should be returning an error, but isn't\n{delete_user_result:#?}");
    }

    // Navidrome returns an error in the form of direct test instead of a proper JSON and therefore
    // doesn't obey the Subsonic or OpenSubsonic API
}

#[tokio::test]
async fn chat_messages() {
    let message = "this is cool stuff";

    // For Subsonic
    let parameters = SubsonicParameters::hashed_password("subsonic rust", SUBSONIC.username, SUBSONIC.password, "1.16.0");
    let subsonic_client = SubsonicClient::new(SUBSONIC.url, parameters);

    subsonic_client.add_chat_message(message).await.unwrap();
    let messages = subsonic_client.get_chat_messages(None).await.unwrap();
    let filtered_messages: Vec<&ChatMessage> = messages.chat_message.iter().filter(|i| i.message.as_ref() == message).collect();
    assert!(!filtered_messages.is_empty(), "No messages found matching the preset one\n{:#?}", messages);

    // For Subsonic
    let parameters = SubsonicParameters::token("subsonic rust", AMPACHE.token, "1.16.0");
    let subsonic_client = OpenSubsonicClient::new(AMPACHE.url, parameters);

    let add_chat_message_result = subsonic_client.add_chat_message(message).await;
    if let Err(SubsonicError::Failed(e)) = add_chat_message_result {
        assert_eq!(e.code, SubsonicErrorCode::GenericError);
    } else {
        panic!("Ampache should be returning an error object but either sent a working response or errored otherwise\n{:#?}", add_chat_message_result);
    }

    let messages = subsonic_client.get_chat_messages(None).await.unwrap();
    assert!(messages.chat_message.is_empty(), "Received Chat Messages even though the server doesn't support them\n{:#?}", messages);
}
