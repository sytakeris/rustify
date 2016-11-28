#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
}
