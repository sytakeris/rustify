extern crate reqwest;
extern crate ansi_term;

use self::reqwest::header::{Authorization, Basic};
use self::ansi_term::Colour::Red;

use api::request::api_request::AuthorizeRequest;
use api::response::api_response::AuthorizeResponse;

use client::tracks::*;

use http::http::Http;

pub struct SpotifyClient {
    pub tracks: Tracks,
    token: String
}

impl SpotifyClient {
    fn new(token: String) -> SpotifyClient {
        SpotifyClient { token: token, tracks: Tracks {} }
    }
}

pub fn authorize(request: AuthorizeRequest) -> SpotifyClient {
    println!("{}: {:#?}",Red.bold().paint("[DEBUG] Sending auth request"), request);

    let params = vec![("grant_type", request.grant_type)];
    let header = Authorization(
        Basic {
            username: request.client_id,
            password: Some(request.client_secret)
        }
    );

    let auth_response: AuthorizeResponse = Http::post("https://accounts.spotify.com/api/token",
                                                     Some(&params),
                                                     Some(header));

    println!("{}: {:#?}",Red.bold().paint("[DEBUG] Got auth response"), auth_response);

    SpotifyClient::new(auth_response.access_token)
}