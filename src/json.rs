use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ScrobbleReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albumartists: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Track {
    pub artists: Vec<String>,
    pub title: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Error {
    #[serde(rename = "type")]
    pub _type: String,
    pub desc: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ScrobbleRes {
    pub status: String,
    pub desc: Option<String>,
    pub track: Option<Track>,
    pub error: Option<Error>,
}
