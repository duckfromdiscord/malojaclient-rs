use crate::{json::*, range::{Range, process_range}, types::*};
use crate::{MalojaCredentials, RequestError, handle_response, get_client_async};
use chrono::prelude::*;
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct Scrobble {
    pub time: DateTime<Utc>,
    pub track: Track,
}

pub async fn scrobbles_async(artist: Option<String>, range: Range, page_number: Option<u64>, scrobbles_per_page: Option<u64>, credentials: MalojaCredentials, client: Client) -> Result<Vec<Scrobble>, RequestError> {
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
        .get(credentials.get_url() + "/apis/mlj_1/scrobbles")
        .json(&requestbody)
        .send()
        .await;
    match handle_response::<ScrobblesRes>(response).await {
        Err(error) => {
            Err(error)
        },
        Ok(response) => {
            let mut scrobbles: Vec<Scrobble> = vec![];
            for scrobble in response.list.unwrap() {
                let dt: DateTime<Utc> = DateTime::from_timestamp(scrobble.time.try_into().unwrap(), 0).unwrap();
                scrobbles.push(Scrobble { time: dt, track: Track::from_trackres(scrobble.track, None) });
            }
            Ok(scrobbles)
        }
    }
}

pub fn scrobbles(artist: Option<String>, range: Range, page_number: Option<u64>, scrobbles_per_page: Option<u64>, credentials: MalojaCredentials) -> Result<Vec<Scrobble>, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on( async {
        let client = get_client_async(&credentials);
        scrobbles_async(artist, range, page_number, scrobbles_per_page, credentials, client).await
    })
}
