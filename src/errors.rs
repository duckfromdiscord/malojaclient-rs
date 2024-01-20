#[derive(Debug)]
pub enum RequestError {
    LocalError(reqwest::Error),
    ServerError(String),
}

#[derive(Debug)]
pub enum MalojaCredentialsBuilderError {
    MissingIP,
    MissingPort,
}