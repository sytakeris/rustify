extern crate reqwest;
extern crate ansi_term;

use api::response::api_response::Track;

use self::ansi_term::Colour::Red;

use http::http::Http;

pub struct Tracks;

pub trait TrackClient {
    fn get_track(&self, id: &str) -> Track;
    fn get_several_tracks(&self, id: &[&str]);
}

impl TrackClient for Tracks {
    fn get_track(&self, id: &str) -> Track {
        println!("{}: {}",Red.bold().paint("[DEBUG] Get track with id"), id);

        let mut track_uri = String::from("https://api.spotify.com/v1/tracks/");
        track_uri.push_str(id);

        let track: Track = Http::get(&track_uri, None);

        println!("{}: {:#?}", Red.bold().paint("[DEBUG] Got track response"), track);

        track
    }

    fn get_several_tracks(&self, id: &[&str]) {
        unimplemented!()
    }
}