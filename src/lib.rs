#![warn(missing_docs)]

//! mljcl: an unofficial client for maloja, a fully self-hosted scrobble server.
//!
//! Given just a maloja server IP and port, mljcl can allow you to query your scrobble history.
//! It can also fetch album art.
//! 
//! With an API key, **mljcl can submit scrobbles to a maloja server**.
//!
//! To use mljcl's functions, you will at least need `MalojaCredentials`.
//! This doesn't mean necessarily that you need an API key,
//! but you do need to have a server to connect to.
//!
//! Here's a basic example of how one might create `MalojaCredentials` for a HTTP server on localhost:
//!
//! ```
//! let creds = mljcl::credentials::MalojaCredentialsBuilder::new()
//!     .https(false)
//!     .ip("127.0.0.1".into())
//!     .port(42010)
//!     .build()
//!     .unwrap();
//! ```
//!
//! You can then, for example, get today's scrobbles for a certain artist:
//!
//! ```
//! let artist = "Some artist";
//! println!("Today's scrobbles for {}: {}", artist, mljcl::history::numscrobbles(
//!     Some(artist.to_string()),
//!     Range::In("today".to_string()),
//!     creds
//! ).unwrap());
//! ```
//! 
//! All async variants of functions require a `reqwest::Client`, which [get_client_async] provides.

/// Creating and using server API credentials, as well as the IP and port of the server.
pub mod credentials;
/// Serverside and client side errors when making requests.
pub mod errors;
/// Raw JSON objects used for communicating between server and client.
pub mod json;

/// Types for artists, tracks, and albums.
#[cfg(feature = "full")]
pub mod types;

/// Chart operations for ranking artists, tracks, and albums.
#[cfg(feature = "full")]
pub mod charts;

/// Scrobble history operations.
#[cfg(feature = "full")]
pub mod history;

/// Time ranges for queries.
#[cfg(feature = "full")]
pub mod range;

/// Requesting album and artist art as images.
#[cfg(feature = "full")]
pub mod art;

use crate::credentials::MalojaCredentials;
use crate::errors::RequestError;
use crate::json::{ScrobbleReq, ScrobbleRes};
use std::collections::HashMap;
use std::str::FromStr;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

fn full_query_path<T: for<'de> serde::Serialize>(query: T, path: &str) -> String {
    let qs = serde_qs::to_string(&query).unwrap();
    match qs.is_empty() {
        true => path.to_string(),
        false => path.to_string() + "?" + &qs,
    }
}

fn parse_headers(maybe_headers: Option<HashMap<String, String>>) -> HeaderMap {
    let mut map = HeaderMap::new();
    if let Some(headers) = maybe_headers {
        for key in headers.keys() {
            let header_key = HeaderName::from_str(key);
            let header_value = HeaderValue::from_str(headers.get(key).unwrap());
            if header_key.is_err() || header_value.is_err() {
                continue;
            }
            map.insert(header_key.unwrap(), header_value.unwrap());
        }
    }
    map
}

async fn handle_response<T: crate::json::MalojaResponse + for<'de> serde::Deserialize<'de>>(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<T, RequestError> {
    if response.is_err() {
        return Err(RequestError::ReqwestError(response.err().unwrap()));
    }
    let response = response.unwrap();
    match response.json::<T>().await {
        Err(error) => Err(RequestError::ReqwestError(error)),
        Ok(parsed_response) => match parsed_response.get_error() {
            None => Ok(parsed_response),
            Some(error) => Err(RequestError::ServerError(error.desc)),
        },
    }
}

/// Provides a `reqwest::Client` given `MalojaCredentials`. It is recommended to use
/// this function specifically as it takes into account whether HTTPS certificate
/// verification should be skipped.
/// 
/// This function is provided so that clients can be reused
/// instead of having to create a new client every time a function is called.
/// 
/// # Examples
/// 
/// ```
/// let client = mljcl::get_client_async(&creds);
/// ```
pub fn get_client_async(
    credentials: &MalojaCredentials,
) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(credentials.skip_cert_verification)
        .build()
}

/// See [scrobble].
pub async fn scrobble_async(
    title: String,
    artist: String,
    credentials: MalojaCredentials,
    client: Client,
) -> Result<ScrobbleRes, RequestError> {
    let scrobblebody = ScrobbleReq {
        artist: Some(artist),
        artists: None,
        title,
        album: None,
        albumartists: None,
        duration: None,
        length: None,
        time: None,
        key: credentials.api_key.as_ref().unwrap().to_string(),
    };
    let response = client
        .post(credentials.get_url() + "/apis/mlj_1/newscrobble")
        .headers(parse_headers(credentials.headers))
        .json(&scrobblebody)
        .send()
        .await;
    handle_response::<ScrobbleRes>(response).await
}

/// Submits a scrobble.
pub fn scrobble(
    title: String,
    artist: String,
    credentials: MalojaCredentials,
) -> Result<ScrobbleRes, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = get_client_async(&credentials);
        scrobble_async(title, artist, credentials, client.unwrap()).await
    })
}
