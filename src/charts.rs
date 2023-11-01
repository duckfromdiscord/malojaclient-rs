use crate::json::*;
use crate::{MalojaCredentials, RequestError, handle_response, get_client};

// Dates are in YYYY/MM/DD format.
pub enum Range {
    AllTime,
    In(String),
    FromTo( (String, String) )
}

#[derive(Debug)]
pub struct Artist {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub struct ArtistChart {
    pub artists: Vec<(Artist, u64)>,
}


#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub id: String,
    pub album: Option<String>,
    pub album_artists: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct TrackChart {
    pub tracks: Vec<(Track, u64)>,
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub artists: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct AlbumChart {
    pub albums: Vec<(Album, u64)>,
}


fn process_range(range: Range) -> (Option<String>, Option<String>, Option<String>) {
    let mut from: Option<String> = None;
    let mut until: Option<String> = None;
    let mut _in: Option<String> = None;
    match range {
        Range::AllTime => {
            from = None;
            until = None;
            _in = None;
        },
        Range::In(range) => {
            _in = Some(range);
        }
        Range::FromTo(range) => {
            from = Some(range.0);
            until = Some(range.1);
        },
    };
    return (from, until, _in);
}

pub fn charts_artists(range: Range, credentials: MalojaCredentials) -> Result<ArtistChart, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = ArtistChartReq {
      from: from_until_in.0,
      until: from_until_in.1,
      _in: from_until_in.2,  
    };
    let response = get_client(&credentials)
        .get(credentials.get_url() + "/apis/mlj_1/charts/artists")
        .json(&requestbody)
        .send();
    match handle_response::<ArtistChartRes>(response) {
        Err(error) => {
            Err(error)
        },
        Ok(response) => {
            let mut artists: Vec<(Artist, u64)> = vec![];
            for artist in response.list.unwrap() {
                artists.push((Artist {
                    name: artist.artist,
                    id: artist.artist_id.to_string(),
                }, artist.rank));
            }
            Ok(ArtistChart {
                artists,
            })
        }
    }
}


pub fn charts_tracks(range: Range, artist: Option<String>, credentials: MalojaCredentials) -> Result<TrackChart, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = TrackChartReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
      artist,
    };
    let response = get_client(&credentials)
        .get(credentials.get_url() + "/apis/mlj_1/charts/tracks")
        .json(&requestbody)
        .send();
    match handle_response::<TrackChartRes>(response) {
        Err(error) => {
            Err(error)
        },
        Ok(response) => {
            let mut tracks: Vec<(Track, u64)> = vec![];
            for track in response.list.unwrap() {
                tracks.push((Track {
                    name: track.track.title,
                    id: track.track_id.to_string(),
                    album: track.track.album.clone().map(|album| Some(album.albumtitle)).unwrap_or(None),
                    album_artists: track.track.album.map(|album| Some(album.artists)).unwrap_or(None),
                }, track.rank));
            }
            Ok(TrackChart {
                tracks,
            })
        }
    }
}

pub fn charts_albums(range: Range, artist: Option<String>, credentials: MalojaCredentials) -> Result<AlbumChart, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = AlbumChartReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
      artist,
    };
    let response = get_client(&credentials)
        .get(credentials.get_url() + "/apis/mlj_1/charts/albums")
        .json(&requestbody)
        .send();
    match handle_response::<AlbumChartRes>(response) {
        Err(error) => {
            Err(error)
        },
        Ok(response) => {
            let mut albums: Vec<(Album, u64)> = vec![];
            for album in response.list.unwrap() {
                albums.push((Album {
                    name: album.album.albumtitle,
                    id: album.album_id.to_string(),
                    artists: Some(album.album.artists),
                }, album.rank));
            }
            Ok(AlbumChart {
                albums,
            })
        }
    }
}