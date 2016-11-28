extern crate reqwest;
extern crate ansi_term;

use self::reqwest::header::{Authorization, Basic};
use self::ansi_term::Colour::Red;

use api::requests::authorization::AuthorizeRequest;
use api::responses::authorization::AuthorizeResponse;

use client::tracks::*;

use http::http::Http;

pub struct SpotifyClient {
    pub tracks: Tracks
}

impl SpotifyClient {
    fn new(token: String) -> SpotifyClient {
        SpotifyClient { tracks: Tracks::new(token) }
    }
}

pub fn authorize(request: AuthorizeRequest) -> SpotifyClient {
    debug!("{}: {:#?}",Red.bold().paint("Sending auth requests"), request);

    let params = vec![("grant_type", request.grant_type)];
    let header = Authorization(
        Basic {
            username: request.client_id,
            password: Some(request.client_secret)
        }
    );

    let auth_response: AuthorizeResponse = Http::post("https://accounts.spotify.com/api/token",
                                                     Some(&params),
                                                     Some(header)).unwrap();

    debug!("{}: {:#?}",Red.bold().paint("Got auth responses"), auth_response);

    SpotifyClient::new(auth_response.access_token)
}