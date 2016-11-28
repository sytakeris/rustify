#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioAnalysis {
    bars: Vec<AudioDuration>,
    beats: Vec<AudioDuration>,
    meta: TrackMeta,
    sections: Vec<TrackSections>,
    segments: Vec<TrackSegments>,
    tatums: Vec<AudioDuration>,
    track: TrackData
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioDuration {
    start: f32,
    duration: f32,
    confidence: f32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackMeta {
    analyzer_version : String,
    platform: String,
    detailed_status: String,
    status_code: i8,
    timestamp: i32,
    input_process: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackSections {
    start: f32,
    duration: f32,
    confidence: i8,
    loudness: f32,
    tempo: f32,
    tempo_confidence: f32,
    key: i8,
    key_confidence: f32,
    time_signature: i8,
    time_signature_confidence: i8
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackSegments {
    start: f32,
    duration: f32,
    confidence: f32,
    loudness_start: f32,
    loudness_max_time: f32,
    loudness_max: f32,
    loudness_end: Option<f32>,
    pitches: Vec<f32>,
    timbre: Vec<f32>,
    tatums: Option<Vec<AudioDuration>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackData {
    num_samples: i32,
    duration: i32,
    sample_md5: String,
    offset_seconds: i16,
    window_seconds: i16,
    analysis_sample_rate: i16,
    analysis_channels: i8,
    end_of_fade_in: i16,
    start_of_fade_out: i16,
    loudness: f32,
    tempo: f32,
    tempo_confidence: f32,
    time_signature: i8,
    time_signature_confidence: i8,
    key: i8,
    key_confidence: f32,
    mode: i8,
    mode_confidence: f32,
    codestring: String,
    code_version: f32,
    echoprintstring: String,
    echoprint_version: f32,
    synchstring: String,
    synch_version: f32,
    rhythmstring: String,
    rhythm_version: f32
}
