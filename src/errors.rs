#[derive(Debug)]
pub enum RequestError {
    ReqwestError(reqwest::Error),
    ServerError(String),
}

#[derive(Debug)]
pub enum MalojaCredentialsBuilderError {
    MissingIP,
    MissingPort,
}