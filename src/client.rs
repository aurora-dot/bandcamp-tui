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
    album: Album,
    track: Track,
    is_playing: bool,
    position: Duration,
    stream_url: Url,
}

// we then use above structures to turn a requested page into terminal view
