use crate::json::{TrackRes, TrackResultRes};

#[derive(Clone, Debug)]
pub struct Track {
    pub name: String,
    pub id: Option<String>,
    pub album: Option<String>,
    pub album_artists: Option<Vec<String>>,
}

impl Track {
    pub fn new(name: String, id: Option<String>, album: Option<String>, album_artists: Option<Vec<String>>) -> Self {
        return Self {
            name,
            id,
            album,
            album_artists,
        }
    }
    pub fn from_trackres(track: TrackRes, id: Option<String>) -> Self {
        return Track::new(
            track.title,
            id,
            track.album.clone().map(|album| Some(album.albumtitle)).unwrap_or(None),
            track.album.map(|album| Some(album.artists)).unwrap_or(None),
        );
    }
    pub fn from_trackresultres(track: TrackResultRes) -> Self {
        return Track::from_trackres(track.track, Some(track.track_id.to_string()));
    }
}

#[derive(Debug)]
pub struct Artist {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub artists: Option<Vec<String>>,
}