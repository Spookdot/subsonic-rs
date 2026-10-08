use crate::OpenSubsonicClient;
use super::*;

async fn get_song_list(client: &OpenSubsonicClient) 
    -> Vec<<OpenSubsonic as SubsonicServerInfo>::Child> {
    let response = client.search3(Search3Parameters::query("")).await.unwrap();
    response.song
}

pub async fn get_random_song_id(client: &OpenSubsonicClient) -> Box<str> {
    let songs = get_song_list(client).await;
    assert!(!songs.is_empty(), "Server does not have any songs");

    let random_number = rand::random_range(0..songs.len());
    songs.into_iter().nth(random_number).unwrap().id
}
