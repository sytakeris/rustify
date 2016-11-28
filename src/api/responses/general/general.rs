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
