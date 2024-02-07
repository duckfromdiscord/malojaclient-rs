use crate::errors::RequestError;
use crate::full_query_path;
use crate::{get_client_async, handle_response, parse_headers, MalojaCredentials};
use crate::{
    json::*,
    range::{process_range, Range},
    types::*,
};
use reqwest::Client;

/// A ranked list of artists.
#[derive(Debug, Clone)]
pub struct ArtistChart {
    /// A `Vec` of each artist and their rank.
    pub artists: Vec<(Artist, u64)>,
}

/// A ranked list of tracks.
#[derive(Debug, Clone)]
pub struct TrackChart {
    /// A `Vec` of each track and its rank.
    pub tracks: Vec<(Track, u64)>,
}

/// A ranked list of albums.
#[derive(Debug, Clone)]
pub struct AlbumChart {
    /// A `Vec` of each album and its rank.
    pub albums: Vec<(Album, u64)>,
}

/// See [charts_artists].
pub async fn charts_artists_async(
    range: Range,
    credentials: MalojaCredentials,
    client: Client,
) -> Result<ArtistChart, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = ArtistChartReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
    };
    let response = client
        .get(full_query_path(
            requestbody,
            &(credentials.get_url() + "/apis/mlj_1/charts/artists"),
        ))
        .headers(parse_headers(credentials.headers))
        .send()
        .await;
    match handle_response::<ArtistChartRes>(response).await {
        Err(error) => Err(error),
        Ok(response) => {
            let mut artists: Vec<(Artist, u64)> = vec![];
            for artist in response.list.unwrap() {
                artists.push((
                    Artist {
                        name: artist.artist,
                        id: artist.artist_id.to_string(),
                    },
                    artist.rank,
                ));
            }
            Ok(ArtistChart { artists })
        }
    }
}

/// Fetches a ranked list of most listened artists, given a time frame.
///
/// # Arguments
///
/// * `range` - A time frame.
/// * `credentials` - Your credentials.
///
/// # Examples
/// ```
/// use mljcl::Range;
///
/// let mut top_artists_ranked = mljcl::charts::charts_artists(Range::AllTime, creds).unwrap().artists;
/// top_artists_ranked.truncate(3);
/// let top_artists: Vec<String> = top_artists_ranked
///         .into_iter()
///         .map(|(artist, _)| artist.name)
///         .collect();
/// println!("Your top 3 artists of all time: {}", top_artists.join(", "));
/// ```
pub fn charts_artists(
    range: Range,
    credentials: MalojaCredentials,
) -> Result<ArtistChart, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = get_client_async(&credentials);
        charts_artists_async(range, credentials, client.unwrap()).await
    })
}

/// See [charts_tracks].
pub async fn charts_tracks_async(
    range: Range,
    artist: Option<String>,
    credentials: MalojaCredentials,
    client: Client,
) -> Result<TrackChart, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = TrackChartReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
        artist,
    };
    let response = client
        .get(full_query_path(
            requestbody,
            &(credentials.get_url() + "/apis/mlj_1/charts/tracks"),
        ))
        .headers(parse_headers(credentials.headers))
        .send()
        .await;
    match handle_response::<TrackChartRes>(response).await {
        Err(error) => Err(error),
        Ok(response) => {
            let mut tracks: Vec<(Track, u64)> = vec![];
            for track in response.list.unwrap() {
                tracks.push((Track::from_trackresultres(track.clone()), track.rank));
            }
            Ok(TrackChart { tracks })
        }
    }
}

/// Fetches a ranked list of most listened tracks, optionally from a given artist, given a time frame.
///
/// # Arguments
///
/// * `range` - A time frame.
/// * `artist` - Optionally, the artist you'd like to rank tracks from.
/// * `credentials` - Your credentials.
///
/// # Examples
/// ```
/// use mljcl::Range;
///
/// let mut top_tracks_ranked = mljcl::charts::charts_tracks(Range::AllTime, None, creds).unwrap().tracks;
/// let top_tracks: Vec<String> = top_tracks_ranked
///         .into_iter()
///         .map(|(track, _)| track.name)
///         .collect();
/// println!("Your top 3 tracks of all time: {}", top_tracks.join(", "));
/// ```
pub fn charts_tracks(
    range: Range,
    artist: Option<String>,
    credentials: MalojaCredentials,
) -> Result<TrackChart, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = get_client_async(&credentials);
        charts_tracks_async(range, artist, credentials, client.unwrap()).await
    })
}

/// See [charts_albums].
pub async fn charts_albums_async(
    range: Range,
    artist: Option<String>,
    credentials: MalojaCredentials,
    client: Client,
) -> Result<AlbumChart, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = AlbumChartReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
        artist,
    };
    let response = client
        .get(full_query_path(
            requestbody,
            &(credentials.get_url() + "/apis/mlj_1/charts/albums"),
        ))
        .headers(parse_headers(credentials.headers))
        .send()
        .await;
    match handle_response::<AlbumChartRes>(response).await {
        Err(error) => Err(error),
        Ok(response) => {
            let mut albums: Vec<(Album, u64)> = vec![];
            for album in response.list.unwrap() {
                albums.push((
                    Album {
                        name: album.album.albumtitle,
                        id: album.album_id.to_string(),
                        artists: album.album.artists,
                    },
                    album.rank,
                ));
            }
            Ok(AlbumChart { albums })
        }
    }
}

/// Fetches a ranked list of most listened albums, optionally from a given artist, given a time frame.
///
/// # Arguments
///
/// * `range` - A time frame.
/// * `artist` - Optionally, the artist you'd like to rank albums from.
/// * `credentials` - Your credentials.
///
/// # Examples
/// ```
/// use mljcl::Range;
///
/// let mut top_albums_ranked = mljcl::charts::charts_albums(Range::AllTime, None, creds).unwrap().albums;
/// top_albums_ranked.truncate(3);
/// let top_albums: Vec<String> = top_albums_ranked
///     .into_iter()
///     .map(|(album, _)| album.name)
///     .collect();
/// println!("Your top 3 albums of all time: {}", top_albums.join(", "));
/// ```
pub fn charts_albums(
    range: Range,
    artist: Option<String>,
    credentials: MalojaCredentials,
) -> Result<AlbumChart, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = get_client_async(&credentials);
        charts_albums_async(range, artist, credentials, client.unwrap()).await
    })
}
