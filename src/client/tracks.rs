extern crate reqwest;
extern crate ansi_term;

use api::responses::tracks::audio_features::{AudioFeature, AudioFeatureList};
use api::responses::tracks::track_analysis::AudioAnalysis;
use api::responses::tracks::tracks::{Track, TrackList};

use self::ansi_term::Colour::Red;

use http::http::Http;

pub struct Tracks {
    token: String
}

impl Tracks {
    pub fn new(token: String) -> Tracks {
        Tracks {token: token}
    }
}

pub trait TrackClient {
    fn get_track(&self, id: &str) -> Track;
    fn get_several_tracks(&self, ids: Vec<&str>) -> TrackList;
}

pub trait TrackClientAuth {
    fn get_audio_analysis(&self, id: &str) -> AudioAnalysis;
    fn get_audio_feature(&self, id: &str) -> AudioFeature;
    fn get_audio_features(&self, ids: Vec<&str>) -> AudioFeatureList;
}

impl TrackClient for Tracks {
    fn get_track(&self, id: &str) -> Track {
        debug!("{}: {}",Red.bold().paint("Get track with id"), id);

        let mut track_uri = String::from("https://api.spotify.com/v1/tracks/");
        track_uri.push_str(id);

        let track: Track = Http::get(&track_uri, None).unwrap();

        debug!("{}: {:#?}", Red.bold().paint("Got track response"), track);

        track
    }

    fn get_several_tracks(&self, ids: Vec<&str>) -> TrackList {
        debug!("{}: {:?}",Red.bold().paint("Get tracks with ids"), ids);

        let params: Vec<_> = ids.iter().map(|track_id| ("ids", *track_id)).collect();

        let track: TrackList = Http::get("https://api.spotify.com/v1/tracks/", Some(params)).unwrap();

        debug!("{}: {:#?}", Red.bold().paint("Got track response"), track);

        track
    }
}

impl TrackClientAuth for Tracks {
    fn get_audio_analysis(&self, id: &str) -> AudioAnalysis {
        debug!("{}: {}",Red.bold().paint("Get track analysis with id"), id);

        let mut track_uri = String::from("https://api.spotify.com/v1/audio-analysis/");
        track_uri.push_str(id);

        let analysis: AudioAnalysis = Http::get_with_auth(&track_uri, None, &self.token).unwrap();

        debug!("{}: {:#?}", Red.bold().paint("Got analysis response"), analysis);

        analysis
    }

    fn get_audio_feature(&self, id: &str) -> AudioFeature {
        unimplemented!()
    }

    fn get_audio_features(&self, ids: Vec<&str>) -> AudioFeatureList {
        unimplemented!()
    }
}