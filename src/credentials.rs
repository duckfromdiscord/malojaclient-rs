use std::collections::HashMap;

use crate::errors::MalojaCredentialsBuilderError;

/// A set of credentials for a maloja server.
#[derive(Debug, Clone)]
pub struct MalojaCredentials {
    /// Whether the server uses HTTPS.
    pub https: bool,
    /// Whether requests to this server should ignore self-signed certificate errors.
    pub skip_cert_verification: bool,
    /// The IP address or hostname of the server.
    pub ip: String,
    /// The port on which the maloja server is listening.
    pub port: u16,
    /// Optionally, the path to the maloja server, in case it is not at `/`.
    pub path: Option<String>,
    /// Optionally, headers to send to the maloja server.
    pub headers: Option<HashMap<String, String>>,
    /// Optionally, a maloja API key.
    pub api_key: Option<String>,
}

impl MalojaCredentials {
    /// Gets a URL to the maloja server given the maloja credentials.
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

    /// Creates a builder for `MalojaCredentials`.
    pub fn builder() -> MalojaCredentialsBuilder {
        MalojaCredentialsBuilder::default()
    }
}

#[derive(Default)]
/// A builder for `MalojaCredentials`, and the recommended way to create the credentials.
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
    /// Initializes a blank `MalojaCredentialsBuilder` which needs to be given an IP and port.
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

    /// Determines whether requests to this server use HTTPS.
    pub fn https(mut self, https: bool) -> MalojaCredentialsBuilder {
        self.https = https;
        self
    }

    /// Determines whether HTTPS requests to this server should ignore certificate errors.
    pub fn skip_cert_verification(
        mut self,
        skip_cert_verification: bool,
    ) -> MalojaCredentialsBuilder {
        self.skip_cert_verification = skip_cert_verification;
        self
    }

    /// Set the server IP.
    pub fn ip(mut self, ip: String) -> MalojaCredentialsBuilder {
        self.ip = Some(ip);
        self
    }

    /// Set the server port.
    pub fn port(mut self, port: u16) -> MalojaCredentialsBuilder {
        self.port = Some(port);
        self
    }

    /// Set the server path.
    pub fn path(mut self, path: String) -> MalojaCredentialsBuilder {
        self.path = Some(path);
        self
    }

    /// Provide headers for server requests.
    pub fn headers(mut self, headers: HashMap<String, String>) -> MalojaCredentialsBuilder {
        self.headers = Some(headers);
        self
    }

    /// Set the API key to use if scrobbling.
    pub fn api_key(mut self, api_key: String) -> MalojaCredentialsBuilder {
        self.api_key = Some(api_key);
        self
    }

    /// Builds the credentials, returning either the credentials or an error.
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
