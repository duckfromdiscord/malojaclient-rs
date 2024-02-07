use crate::json::{TrackRes, TrackResultRes};

/// Track data.
#[derive(Clone, Debug)]
pub struct Track {
    /// The name of the track.
    pub name: String,
    /// The internal maloja ID of the track.
    pub id: Option<String>,
    /// Optionally, the album this track is in.
    pub album: Option<String>,
    /// Optionally, a `Vec` of this track's album's artists.
    pub album_artists: Option<Vec<String>>,
    /// This track's artists.
    pub artists: Vec<String>,
}

#[allow(missing_docs)]
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

/// Artist data.
#[derive(Debug, Clone)]
pub struct Artist {
    /// The name of the artist.
    pub name: String,
    /// The internal maloja ID of the artist.
    pub id: String,
}

/// Album data.
#[derive(Debug, Clone)]
pub struct Album {
    /// The name of the album.
    pub name: String,
    /// The internal maloja ID of the album.
    pub id: String,
    /// Optionally, a `Vec` of this album's artists.
    pub artists: Option<Vec<String>>,
}
