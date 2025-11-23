use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct Tag {
    name: String,
}

#[derive(Debug, Clone)]
pub struct Track {
    id: u16,
    title: String,
    cover: Url,
    lyrics: String,
    tags: Vec<Tag>,
    url: Url,
}

#[derive(Debug, Clone)]
pub struct Album {
    id: u16,
    title: String,
    cover: Url,
    tracks: Vec<Track>,
    tags: Vec<Tag>,
    credits: String,
    url: Url,
}

#[derive(Debug, Clone)]
pub struct Artist {
    id: u16,
    name: String,
    albums: Vec<Album>,
    url: Url,
}

#[derive(Debug, Clone)]
pub struct Player {
    album: Option<Album>,
    track: Option<Track>,
    is_playing: bool,
    position: Option<Duration>,
    stream_url: Option<Url>,
}
