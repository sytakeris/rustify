use api::responses::general::general::{Image, ExternalUrls};
use api::responses::artists::artists::Artist;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename="type")]
    pub general_type: String,
    pub uri: String,
}
