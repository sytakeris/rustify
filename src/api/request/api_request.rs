#[derive(Debug)]
pub struct AuthorizeRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

