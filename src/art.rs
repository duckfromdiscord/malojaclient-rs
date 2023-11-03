use bytes::Bytes;
use crate::{MalojaCredentials, RequestError, get_client};

fn get_image_jpeg(id: String, from_type: &str, credentials: MalojaCredentials) -> Result<Bytes, RequestError> {
    let response = get_client(&credentials)
    .get(credentials.get_url() + "/image?" + from_type + "_id=" + &id)
    .send();
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
                    Ok(actual_response.bytes().unwrap())
                }
            }
        }
    }
}

pub fn album_art_jpeg(id: String, credentials: MalojaCredentials) -> Result<Bytes, RequestError> {
    get_image_jpeg(id, "album", credentials)
}

pub fn artist_art_jpeg(id: String, credentials: MalojaCredentials) -> Result<Bytes, RequestError> {
    get_image_jpeg(id, "artist", credentials)
}
