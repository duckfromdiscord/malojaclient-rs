use crate::{json::*, range::{Range, process_range}, types::*};
use crate::{MalojaCredentials, RequestError, handle_response, get_client};

#[derive(Debug)]
pub struct ArtistChart {
    pub artists: Vec<(Artist, u64)>,
}

#[derive(Debug)]
pub struct TrackChart {
    pub tracks: Vec<(Track, u64)>,
}

#[derive(Debug)]
pub struct AlbumChart {
    pub albums: Vec<(Album, u64)>,
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
                tracks.push((Track::from_trackresultres(track.clone()), track.rank));
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
                    artists: album.album.artists,
                }, album.rank));
            }
            Ok(AlbumChart {
                albums,
            })
        }
    }
}