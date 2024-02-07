#![allow(missing_docs)]

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Error {
    #[serde(rename = "type")]
    pub _type: String,
    pub desc: String,
}

pub trait MalojaResponse {
    fn get_error(&self) -> Option<Error>;
}

macro_rules! impl_malojaresponse {
    ($($names:ident)+) => {
        $(impl MalojaResponse for $names { fn get_error(&self) -> Option<Error> { self.error.clone() } })+
   }
}

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
pub struct ScrobbleResTrack {
    pub artists: Vec<String>,
    pub title: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ScrobbleRes {
    pub status: String,
    pub desc: Option<String>,
    pub track: Option<ScrobbleResTrack>,
    pub error: Option<Error>,
}

impl_malojaresponse!(ScrobbleRes);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ArtistChartReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in")]
    pub _in: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TrackChartReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in")]
    pub _in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ArtistRes {
    pub scrobbles: u64,
    pub real_scrobbles: u64,
    pub artist: String,
    pub artist_id: u64,
    pub rank: u64,
    pub associated_artists: Vec<ArtistRes>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ArtistChartRes {
    pub status: String,
    pub list: Option<Vec<ArtistRes>>,
    pub error: Option<Error>,
}

impl_malojaresponse!(ArtistChartRes);

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct AlbumRes {
    pub artists: Option<Vec<String>>,
    pub albumtitle: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TrackRes {
    pub artists: Vec<String>,
    pub title: String,
    pub album: Option<AlbumRes>,
    pub length: Option<u64>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TrackResultRes {
    pub scrobbles: u64,
    pub track: TrackRes,
    pub track_id: u64,
    pub rank: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TrackChartRes {
    pub status: String,
    pub list: Option<Vec<TrackResultRes>>,
    pub error: Option<Error>,
}

impl_malojaresponse!(TrackChartRes);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AlbumChartReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in")]
    pub _in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AlbumResultRes {
    pub scrobbles: u64,
    pub album: AlbumRes,
    pub album_id: u64,
    pub rank: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AlbumChartRes {
    pub status: String,
    pub list: Option<Vec<AlbumResultRes>>,
    pub error: Option<Error>,
}

impl_malojaresponse!(AlbumChartRes);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ScrobblesReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in")]
    pub _in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perpage: Option<u64>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ScrobblesTrackRes {
    pub time: u64,
    // Thankfully, `/scrobbles` uses the same Track as `/charts/tracks`
    pub track: TrackRes,
    pub duration: Option<u64>,
    pub origin: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ScrobblesRes {
    pub status: String,
    pub list: Option<Vec<ScrobblesTrackRes>>,
    pub error: Option<Error>,
}

impl_malojaresponse!(ScrobblesRes);

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct NumscrobblesRes {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    pub error: Option<Error>,
}

impl_malojaresponse!(NumscrobblesRes);
