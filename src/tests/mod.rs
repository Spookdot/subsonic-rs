use futures::StreamExt;

use crate::{opensubsonic::{OpenSubsonic, models::{EnhancedLyricsList, LyricsList, OpenSubsonicExtension}}, subsonic::Subsonic};

use super::*;

mod ampache;
mod navidrome;
mod subsonic;

pub struct SubsonicLogin<'a> {
    pub url: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

pub struct SubsonicLoginViaToken<'a> {
    pub url: &'a str,
    pub token: &'a str,
}

const TEST_PLAYLIST_NAME: &str = "spooky-rust-subsonic-test";

async fn ping<T: SubsonicServerInfo>(client: &Client<T>) {
    client.ping().await.unwrap();
}

async fn get_license<T: SubsonicServerInfo>(client: &Client<T>) {
    let license = client.get_license().await.unwrap();
    assert!(license.valid, "{:#?}", license);
}

async fn search<T: SubsonicServerInfo>(client: &Client<T>, parameters: SearchParameters) -> T::SearchResult {
    client.search(parameters).await.unwrap()
}

async fn search2<T: SubsonicServerInfo>(client: &Client<T>, parameters: Search3Parameters) -> T::SearchResult2 {
    client.search2(parameters).await.unwrap()
}

async fn search3<T: SubsonicServerInfo>(client: &Client<T>, parameters: Search3Parameters) -> T::SearchResult3 {
    client.search3(parameters).await.unwrap()
}

async fn star_unstar_song_subsonic(client: &SubsonicClient) {
    let search3_response = client.search3(Search3Parameters::query("e")).await.unwrap();
    let song_id = search3_response.song[0].id.as_ref();

    // Unstar to return to it to a default state
    client.unstar(StarParameters::id(song_id)).await.unwrap();

    // Star and check if it's starred
    client.star(StarParameters::id(song_id)).await.unwrap();
    let song = client.get_song(song_id).await.unwrap();
    assert!(song.starred.is_some(), "Song should be starred after Star method was called");

    // Unstar and check if it's unstarred
    client.unstar(StarParameters::id(song_id)).await.unwrap();
    let song = client.get_song(song_id).await.unwrap();
    assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");
}

async fn star_unstar_song_opensubsonic(client: &OpenSubsonicClient) {
    let search3_response = client.search3(Search3Parameters::query("")).await.unwrap();
    let song_id = search3_response.song[0].id.as_ref();

    // Unstar to return to it to a default state
    client.unstar(StarParameters::id(song_id)).await.unwrap();

    // Star and check if it's starred
    client.star(StarParameters::id(song_id)).await.unwrap();
    let song = client.get_song(song_id).await.unwrap();
    assert!(song.starred.is_some(), "Song should be starred after Star method was called");

    // Unstar and check if it's unstarred
    client.unstar(StarParameters::id(song_id)).await.unwrap();
    let song = client.get_song(song_id).await.unwrap();
    assert!(song.starred.is_none(), "Song should be unstarred after Unstar method was called");
}

async fn get_lyrics<T: SubsonicServerInfo>(client: &Client<T>, parameters: GetLyricsParameters) -> Lyrics {
    client.get_lyrics(parameters).await.unwrap()
}

async fn get_lyrics_by_song_id(client: &OpenSubsonicClient, song_title: &str) -> LyricsList {
    let get_song = client.search3(Search3Parameters::song(song_title, 50)).await.unwrap();
    let song_id = &get_song.song[0].id;
    client.get_lyrics_by_song_id(song_id).await.unwrap()
}

async fn get_lyrics_by_song_id_enhanced(client: &OpenSubsonicClient, song_title: &str) -> EnhancedLyricsList {
    let get_song = client.search3(Search3Parameters::song(song_title, 50)).await.unwrap();
    let song_id = &get_song.song[0].id;
    client.get_lyrics_by_song_id_enhanced(song_id).await.unwrap()
}

async fn get_opensubsonic_extensions(client: &OpenSubsonicClient) -> Vec<OpenSubsonicExtension> {
    client.get_open_subsonic_extensions().await.unwrap()
}

async fn get_music_folders<T: SubsonicServerInfo>(client: &Client<T>) -> MusicFolders {
    client.get_music_folders().await.unwrap()
}

async fn create_user<T: SubsonicServerInfo>(client: &Client<T>) -> T::ErrorData {
    let result = client.create_user(CreateUserParameters::with_default_roles("test", "test", "test")).await;

    if let Err(SubsonicError::Failed(error_data)) = result {
        error_data
    } else {
        panic!("createUser endpoint for Subsonic should be returning an error, but isn't\n{result:#?}");
    }
}

async fn delete_user<T: SubsonicServerInfo>(client: &Client<T>) -> T::ErrorData {
    let result = client.delete_user("test").await;

    if let Err(SubsonicError::Failed(error_data)) = result {
        error_data
    } else {
        panic!("createUser endpoint for Subsonic should be returning an error, but isn't\n{result:#?}");
    }
}

async fn chat_messages<T: SubsonicServerInfo>(client: &Client<T>) {
    let message = "this is cool stuff";
    client.add_chat_message(message).await.unwrap();
    let messages = client.get_chat_messages(None).await.unwrap();
    let filtered_messages: Vec<&ChatMessage> = messages.chat_message.iter().filter(|i| i.message.as_ref() == message).collect();
    assert!(!filtered_messages.is_empty(), "No messages found matching the preset one\n{:#?}", messages);
}

async fn change_password<T: SubsonicServerInfo>(client: &Client<T>, username: &str, password: &str) -> T::ErrorData {
    let result = client.change_password(username, password).await;
    if let Err(SubsonicError::Failed(error_data)) = result {
        error_data
    } else {
        panic!("Received wrong error or an Ok where a SubsonicError::Failed was expected\n{:#?}", result);
    }
}

async fn bookmarks_subsonic(client: &SubsonicClient) {
    let search3_response = client.search3(Search3Parameters::query("e")).await.unwrap();
    let song_id = search3_response.song[0].id.as_ref();
    
    // Check current bookmark status
    let bookmarks = client.get_bookmarks().await.unwrap();
    let is_bookmarked = bookmarks.bookmark.into_iter().find(|item| item.entry.id.as_ref() == song_id).is_some();
    // Toggle bookmark
    if is_bookmarked {
        client.delete_bookmark(song_id).await.unwrap();
    } else {
        client.create_bookmark(CreateBookmarkParameters::id(song_id)).await.unwrap();
    }
}

async fn _bookmarks_opensubsonic(client: &OpenSubsonicClient) {
    let search3_response = client.search3(Search3Parameters::query("")).await.unwrap();
    let song_id = search3_response.song[0].id.as_ref();
    
    // Check current bookmark status
    let bookmarks = client.get_bookmarks().await.unwrap();
    let is_bookmarked = bookmarks.bookmark.into_iter().find(|item| item.entry.id.as_ref() == song_id).is_some();
    // Toggle bookmark
    if is_bookmarked {
        client.delete_bookmark(song_id).await.unwrap();
    } else {
        client.create_bookmark(CreateBookmarkParameters::id(song_id)).await.unwrap();
    }
}

async fn create_internet_radio_stations<T: SubsonicServerInfo>(client: &Client<T>) -> T::ErrorData {
    let parameters = CreateInternetRadioStationParameters::without_homepage("e", "e");
    let result = client.create_internet_radio_station(parameters).await;
    if let Err(SubsonicError::Failed(error_data)) = result {
        error_data
    } else {
        panic!("Received wrong error or an Ok where a SubsonicError::Failed was expected\n{:#?}", result);
    }
}

async fn delete_internet_radio_stations<T: SubsonicServerInfo>(client: &Client<T>) -> T::ErrorData {
    let result = client.delete_internet_radio_station("e").await;
    if let Err(SubsonicError::Failed(error_data)) = result {
        error_data
    } else {
        panic!("Received wrong error or an Ok where a SubsonicError::Failed was expected\n{:#?}", result);
    }
}

async fn get_internet_radio_stations<T: SubsonicServerInfo>(client: &Client<T>) -> T::InternetRadioStations {
    client.get_internet_radio_stations().await.unwrap()
}

async fn opensubsonic_playlists(client: &OpenSubsonicClient) {
    // Check if the playlist we are creating for testing already exists
    let existing_playlists = client.get_playlists(None).await.unwrap();
    let playlist_option = existing_playlists
        .playlist
        .into_iter()
        .find(|item| item.name.as_ref() == TEST_PLAYLIST_NAME);
    if let Some(playlist) = playlist_option {
        // Delete the playlist if it already exists
        client.delete_playlist(&playlist.id).await.unwrap();
    }
    
    // Create our playlist
    let created_playlist = client
        .create_playlist(CreatePlaylistParameters::name(TEST_PLAYLIST_NAME))
        .await
        .unwrap();

    // Get the id for our created playlist
    let existing_playlists = client.get_playlists(None).await.unwrap();
    let playlist = existing_playlists
        .playlist
        .into_iter()
        .find(|item| item.name.as_ref() == TEST_PLAYLIST_NAME)
        .unwrap();

    assert_eq!(created_playlist.id, playlist.id);
    
    // Test if the get_playlist function works
    client.get_playlist(&playlist.id).await.unwrap();

    // delete the playlist again
    client.delete_playlist(&playlist.id).await.unwrap();
}

async fn subsonic_playlists(client: &SubsonicClient) {
    // Check if the playlist we are creating for testing already exists
    let existing_playlists = client.get_playlists(None).await.unwrap();
    let playlist_option = existing_playlists
        .playlist
        .into_iter()
        .find(|item| item.name.as_ref() == TEST_PLAYLIST_NAME);
    if let Some(playlist) = playlist_option {
        // Delete the playlist if it already exists
        client.delete_playlist(&playlist.id).await.unwrap();
    }
    
    // Create our playlist
    let created_playlist = client
        .create_playlist(CreatePlaylistParameters::name(TEST_PLAYLIST_NAME))
        .await
        .unwrap();

    // Get the id for our created playlist
    let existing_playlists = client.get_playlists(None).await.unwrap();
    let playlist = existing_playlists
        .playlist
        .into_iter()
        .find(|item| item.name.as_ref() == TEST_PLAYLIST_NAME)
        .unwrap();

    assert_eq!(created_playlist.id, playlist.id);
    
    // Test if the get_playlist function works
    client.get_playlist(&playlist.id).await.unwrap();

    // delete the playlist again
    client.delete_playlist(&playlist.id).await.unwrap();
}

async fn podcasts<T: SubsonicServerInfo>(client: &Client<T>) {
    client.get_podcasts(GetPodcastsParameters::default()).await.unwrap();
    client.get_podcasts(GetPodcastsParameters::include_episodes(false)).await.unwrap();
}

async fn subsonic_shares(client: &SubsonicClient) 
    -> Result<(), SubsonicError<<Subsonic as traits::SubsonicServerInfo>::ErrorData>> 
{
    let search3_response = client.search3(Search3Parameters::query("e")).await?;
    let song_id = search3_response.song[0].id.as_ref();
    
    // Create a Share
    let share = client.create_share(CreateShareParameters::one(song_id)).await?;
    let share_id = share.share[0].id.as_ref();

    // Check that said share now exists
    let shares = client.get_shares().await?;
    let filtered_shares: Vec<_> = shares.share.into_iter().filter(|x| x.id.as_ref() == share_id).collect();
    assert!(!filtered_shares.is_empty());

    // Delete Share
    client.delete_share(share_id).await?;

    // Check that said share doesn't exist anymore
    let shares = client.get_shares().await?;
    let filtered_shares: Vec<_> = shares.share.into_iter().filter(|x| x.id.as_ref() == share_id).collect();
    assert!(filtered_shares.is_empty());
    Ok(())
}

async fn opensubsonic_shares(client: &OpenSubsonicClient) 
    -> Result<(), SubsonicError<<OpenSubsonic as traits::SubsonicServerInfo>::ErrorData>> 
{
    let search3_response = client.search3(Search3Parameters::query("")).await?;
    let song_id = search3_response.song[0].id.as_ref();
    
    // Create a Share
    let share = client.create_share(CreateShareParameters::one(song_id)).await?;
    assert!(!share.share.is_empty(), "{share:#?}");
    let share_id = share.share[0].id.as_ref();

    // Check that said share now exists
    let shares = client.get_shares().await?;
    let filtered_shares: Vec<_> = shares.share.into_iter().filter(|x| x.id.as_ref() == share_id).collect();
    assert!(!filtered_shares.is_empty());

    // Delete Share
    client.delete_share(share_id).await?;

    // Check that said share doesn't exist anymore
    let shares = client.get_shares().await?;
    let filtered_shares: Vec<_> = shares.share.into_iter().filter(|x| x.id.as_ref() == share_id).collect();
    assert!(filtered_shares.is_empty());
    Ok(())
}

async fn download_intentional_error<T: SubsonicServerInfo>(client: &Client<T>) -> T::ErrorData {
    let mut result = client.download("").await;
    if let Err(SubsonicError::Failed(error_data)) = result {
        error_data
    } else {
        if let Ok(result) = result.as_mut() {
            panic!(
                "{:?}\n{:?}", 
                std::any::type_name_of_val(result),
                result.next().await
            );
        }
        let error = result.err().unwrap();
        panic!("{error:?}");
    }
}

async fn subsonic_download(client: &SubsonicClient) {
    let search3_response = client.search3(Search3Parameters::query("e")).await.unwrap();
    let song_id = search3_response.song[0].id.as_ref();

    let stream = client.download(song_id).await.unwrap();
    assert_ne!(stream.count().await, 0, "Stream is empty");
}

async fn opensubsonic_download(client: &OpenSubsonicClient) {
    let search3_response = client.search3(Search3Parameters::query("")).await.unwrap();
    let song_id = search3_response.song[0].id.as_ref();

    let stream = client.download(song_id).await.unwrap();
    assert_ne!(stream.count().await, 0, "Stream is empty");
}
