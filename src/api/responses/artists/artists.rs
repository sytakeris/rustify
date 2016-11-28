use api::responses::general::general::ExternalUrls;

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
