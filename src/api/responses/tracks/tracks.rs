use api::responses::general::general::{ExternalIds, ExternalUrls};
use api::responses::albums::albums::Album;
use api::responses::artists::artists::Artist;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackList {
    pub tracks: Vec<Track>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub popularity: i32,
    pub preview_url: String,
    #[serde(rename="type")]
    pub general_type: String,
    pub uri: String
}