use crate::errors::RequestError;
use crate::full_query_path;
use crate::{get_client_async, handle_response, parse_headers, MalojaCredentials};
use crate::{
    json::*,
    range::{process_range, Range},
    types::*,
};
use chrono::prelude::*;
use reqwest::Client;

/// A scrobble: track and timestamp.
#[derive(Clone, Debug)]
pub struct Scrobble {
    /// The `DateTime<Utc>` when this track was played.
    pub time: DateTime<Utc>,
    /// Track data.
    pub track: Track,
}

/// See [scrobbles].
pub async fn scrobbles_async(
    artist: Option<String>,
    range: Range,
    page_number: Option<u64>,
    scrobbles_per_page: Option<u64>,
    credentials: MalojaCredentials,
    client: Client,
) -> Result<Vec<Scrobble>, RequestError> {
    let from_until_in = process_range(range);
    let requestbody = ScrobblesReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
        artist,
        page: page_number,
        perpage: scrobbles_per_page,
    };
    let response = client
        .get(full_query_path(
            requestbody,
            &(credentials.get_url() + "/apis/mlj_1/scrobbles"),
        ))
        .headers(parse_headers(credentials.headers))
        .send()
        .await;
    match handle_response::<ScrobblesRes>(response).await {
        Err(error) => Err(error),
        Ok(response) => {
            let mut scrobbles: Vec<Scrobble> = vec![];
            for scrobble in response.list.unwrap() {
                let dt: DateTime<Utc> =
                    DateTime::from_timestamp(scrobble.time.try_into().unwrap(), 0).unwrap();
                scrobbles.push(Scrobble {
                    time: dt,
                    track: Track::from_trackres(scrobble.track, None),
                });
            }
            Ok(scrobbles)
        }
    }
}

/// Returns a `Vec` of scrobbles within a given time range.
/// 
/// # Arguments
/// 
/// * `artist` - Optionally, an artist to view scrobbles of.
/// * `range` - A time range.
/// * `page_number` - Optionally, the page of scrobbles to view. Will return all scrobbles otherwise. Starts at 0 for most recent.
/// * `scrobbles_per_page` - Optionally, the amount of scrobbles to view per page. Higher will return more scrobbles per request, but requests will take longer.
/// * `credentials` - Your credentials.
/// 
/// # Examples
/// 
/// ```
/// let recent_scrobbles_vec = mljcl::history::scrobbles(None, mljcl::range::Range::AllTime, Some(0), Some(30), creds).unwrap();
/// let recent_scrobbles: Vec<String> = recent_scrobbles_vec
///     .into_iter()
///     .map(|scrobble: history::Scrobble| scrobble.track.artists.join(", ") + " - " + &scrobble.track.name)
///     .collect();
/// println!("Your 30 most recent scrobbles: {}", recent_scrobbles.join("; "));
/// ```
pub fn scrobbles(
    artist: Option<String>,
    range: Range,
    page_number: Option<u64>,
    scrobbles_per_page: Option<u64>,
    credentials: MalojaCredentials,
) -> Result<Vec<Scrobble>, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = get_client_async(&credentials);
        scrobbles_async(
            artist,
            range,
            page_number,
            scrobbles_per_page,
            credentials,
            client.unwrap(),
        )
        .await
    })
}

/// See [numscrobbles].
pub async fn numscrobbles_async(
    artist: Option<String>,
    range: Range,
    credentials: MalojaCredentials,
    client: Client,
) -> Result<u64, RequestError> {
    let from_until_in = process_range(range);
    // numscrobbles uses the same exact documentation/query structure as scrobbles even though pages aren't relevant
    let requestbody = ScrobblesReq {
        from: from_until_in.0,
        until: from_until_in.1,
        _in: from_until_in.2,
        artist,
        page: None,
        perpage: None,
    };
    let response = client
        .get(full_query_path(
            requestbody,
            &(credentials.get_url() + "/apis/mlj_1/numscrobbles"),
        ))
        .headers(parse_headers(credentials.headers))
        .send()
        .await;
    match handle_response::<NumscrobblesRes>(response).await {
        Err(error) => Err(error),
        Ok(response) => match response.amount {
            Some(amount) => Ok(amount),
            None => Err(RequestError::ServerError(response.status)),
        },
    }
}

/// Provides a scrobble count, within a given time range, optionally only for a certain artist.
pub fn numscrobbles(
    artist: Option<String>,
    range: Range,
    credentials: MalojaCredentials,
) -> Result<u64, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = get_client_async(&credentials);
        numscrobbles_async(artist, range, credentials, client.unwrap()).await
    })
}
