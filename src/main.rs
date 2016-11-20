extern crate rustify;
use rustify::client::*;
use rustify::api::request::api_request::{AuthorizeRequest};
use rustify::client::tracks::TrackClient;

fn main() {
    let authorize = AuthorizeRequest {
        grant_type: String::from("client_credentials")
    };

    let api = client::authorize(authorize);
    api.tracks.get_track("4uRHaUn2btiPeatoVmjG0x");
}
