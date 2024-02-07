/// A request error from a `mljcl` function.
#[derive(Debug)]
pub enum RequestError {
    /// An error from the `reqwest` crate when making an HTTP request.
    ReqwestError(reqwest::Error),
    /// A serverside error in maloja.
    ServerError(String),
}

/// An error building `MalojaCredentials`.
#[derive(Debug)]
pub enum MalojaCredentialsBuilderError {
    /// The builder was missing an IP address.
    MissingIP,
    /// The builder was missing a port.
    MissingPort,
}
