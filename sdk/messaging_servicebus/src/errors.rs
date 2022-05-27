/// A specialized Result type.
pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(#[from] azure_core::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] azure_core::ParseError),
    #[error("Permission error: {0}")]
    PermissionError(#[from] azure_core::PermissionError),
    #[error("Parse bool error: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("To str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Date time parse error: {0}")]
    DateTimeParseError(#[from] chrono::format::ParseError),
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),
    #[error("Traversing error: {0}")]
    TraversingError(#[from] azure_core::TraversingError),
    #[error("From UTF-8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Invalid status code: {0:?}")]
    InvalidStatusCode(#[from] http::status::InvalidStatusCode),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("A required header is missing: {0}")]
    MissingHeaderError(String),
    #[error(
        "An expected JSON node is missing: {} of expected type {}",
        value,
        expected_type
    )]
    MissingValueError {
        value: String,
        expected_type: String,
    },
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Header not found: {0}")]
    HeaderNotFound(String),
    #[error("Error parsing the transaction response: {0:?}")]
    TransactionResponseParseError(String),
    #[error("Generic error: {0}")]
    GenericErrorWithText(String),
    #[error("Operation not supported. Operation == {0}, reason == {1}")]
    OperationNotSupported(String, String),
    #[error("UnexpectedXMLError: {0}")]
    UnexpectedXMLError(String),
    #[error("digest length {0} bytes instead of 16")]
    DigestNot16BytesLong(usize),
    #[error("CRC64 length {0} bytes instead of 8")]
    CRC64Not8BytesLong(usize),
    #[error("At least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error("error writing the header value: {0}")]
    InvalidHeaderValue(#[from] azure_core::HttpHeaderError),
    #[error("error generating hmac: {0}")]
    Hmac(#[from] hmac::digest::InvalidLength),
}

impl From<azure_core::error::Error> for Error {
    fn from(err: azure_core::error::Error) -> Self {
        Self::CoreError(err.into())
    }
}

impl From<azure_core::HttpError> for Error {
    fn from(error: azure_core::HttpError) -> Self {
        Self::CoreError(azure_core::Error::Http(error))
    }
}
