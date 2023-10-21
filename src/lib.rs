mod json;

use crate::json::{ScrobbleReq, ScrobbleRes};

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
        return format!("{}{}:{}", protocol, &self.ip, &self.port);
    }
}

pub fn scrobble(title: String, artist: String, credentials: MalojaCredentials) -> Result<ScrobbleRes, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(credentials.skip_cert_verification)
        .build()
        .unwrap();
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
        .json(&scrobblebody)
        .send()
        .unwrap();
    response.json()
}
