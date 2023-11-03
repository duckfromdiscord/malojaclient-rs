mod json;

#[cfg(feature = "full")]
pub mod types;

#[cfg(feature = "full")]
pub mod charts;

#[cfg(feature = "full")]
pub mod history;

#[cfg(feature = "full")]
pub mod range;

#[cfg(feature = "full")]
pub mod art;

use crate::json::{ScrobbleReq, ScrobbleRes};


#[derive(Debug)]
pub enum RequestError {
    LocalError(reqwest::Error),
    ServerError(String),
}

#[derive(Debug, Clone)]
pub struct MalojaCredentials {
    pub https: bool,
    pub skip_cert_verification: bool,
    pub ip: String,
    pub port: u16,
    pub api_key: Option<String>,
}

impl MalojaCredentials {
    pub fn get_url(&self) -> String {
        let protocol = match self.https {
            true => "https://",
            false => "http://",
        };
        format!("{}{}:{}", protocol, &self.ip, &self.port)
    }
}

fn handle_response<T: crate::json::MalojaResponse + for<'de> serde::Deserialize<'de>>(response: Result<reqwest::blocking::Response, reqwest::Error>) -> Result<T, RequestError> {
    if response.is_err() {
        return Err(RequestError::LocalError(response.err().unwrap()));
    }
    let response = response.unwrap();
    match response.json::<T>() {
        Err(error) => {
            Err(RequestError::LocalError(error))
        },
        Ok(parsed_response) => {
            match parsed_response.get_error() {
                None => Ok(parsed_response),
                Some(error) => Err(RequestError::ServerError(error.desc)),
            }
        }
    }
}

pub fn get_client(credentials: &MalojaCredentials) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(credentials.skip_cert_verification)
        .build()
        .unwrap()
}

pub fn scrobble(title: String, artist: String, credentials: MalojaCredentials) -> Result<ScrobbleRes, RequestError> {
    
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
    let response = get_client(&credentials)
        .post(credentials.get_url() + "/apis/mlj_1/newscrobble")
        .json(&scrobblebody)
        .send();
    handle_response::<ScrobbleRes>(response)
}
