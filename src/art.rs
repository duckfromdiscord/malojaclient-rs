use crate::credentials::MalojaCredentials;
use crate::errors::RequestError;
use crate::{get_client_async, parse_headers};
use bytes::Bytes;
use reqwest::Client;

async fn get_image_async(id: String, from_type: &str, credentials: MalojaCredentials, client: Client) -> Result<Bytes, RequestError> {
    let response = client
    .get(credentials.get_url() + "/image?" + from_type + "_id=" + &id)
    .headers(parse_headers(credentials.headers))
    .send()
    .await;
    match response {
        Err(err) => {
            Err(RequestError::LocalError(err))
        },
        Ok(response) => {
            match response.error_for_status() {
                Err(err) => {
                    Err(RequestError::ServerError(err.to_string()))
                },
                Ok(actual_response) => {
                    Ok(actual_response.bytes().await.unwrap())
                }
            }
        }
    }
}

pub async fn album_art_async(id: String, credentials: MalojaCredentials, client: Client) -> Result<Bytes, RequestError> {
    get_image_async(id, "album", credentials, client).await
}

pub async fn artist_art_async(id: String, credentials: MalojaCredentials, client: Client) -> Result<Bytes, RequestError> {
    get_image_async(id, "artist", credentials, client).await
}


fn get_image(id: String, from_type: &str, credentials: MalojaCredentials) -> Result<Bytes, RequestError> {
    tokio::runtime::Runtime::new().unwrap().block_on( async {
        let client = get_client_async(&credentials);
        get_image_async(id, from_type, credentials, client).await
    })
}

pub fn album_art(id: String, credentials: MalojaCredentials) -> Result<Bytes, RequestError> {
    get_image(id, "album", credentials)
}

pub fn artist_art(id: String, credentials: MalojaCredentials) -> Result<Bytes, RequestError> {
    get_image(id, "artist", credentials)
}