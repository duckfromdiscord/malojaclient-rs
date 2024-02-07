use std::collections::HashMap;

use crate::errors::MalojaCredentialsBuilderError;

#[derive(Debug, Clone)]
pub struct MalojaCredentials {
    pub https: bool,
    pub skip_cert_verification: bool,
    pub ip: String,
    pub port: u16,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub api_key: Option<String>,
}

impl MalojaCredentials {
    pub fn get_url(&self) -> String {
        let protocol = match self.https {
            true => "https://",
            false => "http://",
        };
        let mut sub_path = self
            .clone()
            .path
            .unwrap_or("".to_string())
            .trim_matches('/')
            .to_owned();
        if !sub_path.is_empty() {
            sub_path = "/".to_owned() + &sub_path;
        }
        format!("{}{}:{}{}", protocol, &self.ip, &self.port, sub_path)
    }

    pub fn builder() -> MalojaCredentialsBuilder {
        MalojaCredentialsBuilder::default()
    }
}

#[derive(Default)]
pub struct MalojaCredentialsBuilder {
    https: bool,
    skip_cert_verification: bool,
    ip: Option<String>,
    port: Option<u16>,
    path: Option<String>,
    headers: Option<HashMap<String, String>>,
    api_key: Option<String>,
}

impl MalojaCredentialsBuilder {
    pub fn new() -> MalojaCredentialsBuilder {
        MalojaCredentialsBuilder {
            https: false,
            skip_cert_verification: true,
            ip: None,
            port: None,
            path: None,
            headers: None,
            api_key: None,
        }
    }

    pub fn https(mut self, https: bool) -> MalojaCredentialsBuilder {
        self.https = https;
        self
    }

    pub fn skip_cert_verification(
        mut self,
        skip_cert_verification: bool,
    ) -> MalojaCredentialsBuilder {
        self.skip_cert_verification = skip_cert_verification;
        self
    }

    pub fn ip(mut self, ip: String) -> MalojaCredentialsBuilder {
        self.ip = Some(ip);
        self
    }

    pub fn port(mut self, port: u16) -> MalojaCredentialsBuilder {
        self.port = Some(port);
        self
    }

    pub fn path(mut self, path: String) -> MalojaCredentialsBuilder {
        self.path = Some(path);
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> MalojaCredentialsBuilder {
        self.headers = Some(headers);
        self
    }

    pub fn api_key(mut self, api_key: String) -> MalojaCredentialsBuilder {
        self.api_key = Some(api_key);
        self
    }

    pub fn build(self) -> Result<MalojaCredentials, MalojaCredentialsBuilderError> {
        let ip = self.ip.ok_or(MalojaCredentialsBuilderError::MissingIP)?;
        let port = self
            .port
            .ok_or(MalojaCredentialsBuilderError::MissingPort)?;
        Ok(MalojaCredentials {
            https: self.https,
            skip_cert_verification: self.skip_cert_verification,
            ip,
            port,
            path: self.path,
            headers: self.headers,
            api_key: self.api_key,
        })
    }
}
