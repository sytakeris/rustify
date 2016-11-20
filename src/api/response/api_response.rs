#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename="type")]
    pub general_type: String,
    pub uri: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub height: i16,
    pub url: String,
    pub width: i16
}