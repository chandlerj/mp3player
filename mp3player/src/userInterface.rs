use id3::{Tag, TagLike};
use std::fmt;
use crate::AudioPlayer;

pub fn display_track_metadata(currently_playing: &AudioPlayer){
    let song_tags = Tag::read_from_path(currently_playing.file_path.clone()).unwrap();
    let artist = song_tags.artist().unwrap();
    let album = song_tags.album().unwrap();
    let title = song_tags.title().unwrap();
    let now_playing = format!("Now Playing: {} - {} from {}", artist, title, album);
    println!("{now_playing}");
}