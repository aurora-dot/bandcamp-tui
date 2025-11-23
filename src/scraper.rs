use url::Url;
use reqwest::Client;

pub struct Scraper {
    http_client: Client,
}
