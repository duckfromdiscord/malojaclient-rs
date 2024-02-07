use crate::json::{TrackRes, TrackResultRes};

#[derive(Clone, Debug)]
pub struct Track {
    pub name: String,
    pub id: Option<String>,
    pub album: Option<String>,
    pub album_artists: Option<Vec<String>>,
    pub artists: Vec<String>,
}

impl Track {
    pub fn new(
        name: String,
        id: Option<String>,
        album: Option<String>,
        album_artists: Option<Vec<String>>,
        artists: Vec<String>,
    ) -> Self {
        Self {
            name,
            id,
            album,
            album_artists,
            artists,
        }
    }
    pub fn from_trackres(track: TrackRes, id: Option<String>) -> Self {
        Track::new(
            track.title,
            id,
            track
                .album
                .clone()
                .map(|album| Some(album.albumtitle))
                .unwrap_or(None),
            track
                .album
                .map(|album| Some(album.artists))
                .unwrap_or(None)
                .unwrap_or(None),
            track.artists,
        )
    }
    pub fn from_trackresultres(track: TrackResultRes) -> Self {
        Track::from_trackres(track.track, Some(track.track_id.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub artists: Option<Vec<String>>,
}
