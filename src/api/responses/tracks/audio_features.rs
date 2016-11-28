#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioFeature {

}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioFeatureList {
    pub audio_feature: Vec<AudioFeature>
}

