use crate::scraper::Scraper;
use crate::state::{Album, Artist, Player, Track};
use url::Url;

#[derive(Debug, Clone)]
pub enum PageContent {
    Artist(Artist),
    Album(Album),
    Track(Track),
}

pub struct Page {
    url: Url,
    content: PageContent,
}

pub struct Client {
    player: Player,
    scraper: Scraper,
    current_page: Page,
    forward: Vec<Page>,
    back: Vec<Page>,
}
