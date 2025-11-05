use std::collections::HashMap;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidHeaderKey(String),
    InvalidRequestKind(String),
    InvalidFormat(String),
    HeaderMissing(String),
    HeaderEmpty(String),
}

impl From<ParseError> for StatusCode {
    fn from(err: ParseError) -> Self {
        err.to_status_code()
    }
}

impl ParseError {
    // returns a status code and an explanation
    pub fn to_status_code(self) -> StatusCode {
        match self {
            ParseError::InvalidHeaderKey(_) => StatusCode::HeaderInvalid,
            ParseError::InvalidRequestKind(_) => StatusCode::InvalidRequestKind,
            ParseError::InvalidFormat(_) => StatusCode::BadRequest,
            ParseError::HeaderMissing(_) => StatusCode::HeaderMissing,
            ParseError::HeaderEmpty(_) => StatusCode::HeaderEmpty,
        }
    }
    pub fn inner(&self) -> &str {
        match self {
            ParseError::InvalidHeaderKey(s)
            | ParseError::InvalidRequestKind(s)
            | ParseError::InvalidFormat(s)
            | ParseError::HeaderEmpty(s)
            | ParseError::HeaderMissing(s) => s,
        }
    }
}

#[derive(Clone, Debug)]
pub enum StatusCode {
    MessageSent = 1,
    InternalError = -1,
    BadRequest = -10,
    InvalidRequestKind = -11,
    HeaderMissing = -20,
    HeaderInvalid = -21,
    HeaderEmpty = -22,
    AuthInvalid = -90,
    Unsupported = -80,
    ChallengeGiven = 90,
    ChallengeCompleted = 91,
    HashAccepted = 60,
    Teapot = 0,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StatusCode::MessageSent => "message sent",
                StatusCode::InternalError => "internal error",
                StatusCode::BadRequest => "bad request",
                StatusCode::InvalidRequestKind => "invalid request kind",
                StatusCode::HeaderMissing => "header missing",
                StatusCode::HeaderInvalid => "header invalid",
                StatusCode::HeaderEmpty => "header empty",
                StatusCode::AuthInvalid => "auth invalid",
                StatusCode::ChallengeGiven => "challenge given",
                StatusCode::ChallengeCompleted => "challenge completed",
                StatusCode::Teapot => "teapot status",
                StatusCode::Unsupported => "the server does not support this action yet",
                StatusCode::HashAccepted => "hash accepted",
            }
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
pub enum HeaderKind {
    To,
    Through,
    // auth
    Client,
    Session,
    Nonce,
    HMAC,
    ChallengeOk,
    // hash auth
    Hash,
    HashAccepted,
}

impl HeaderKind {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.to_ascii_lowercase().as_str() {
            "to" => Ok(Self::To),
            "through" => Ok(Self::Through),
            "client" => Ok(Self::Client),
            "session" => Ok(Self::Session),
            "nonce" => Ok(Self::Nonce),
            "hmac" => Ok(Self::HMAC),
            "challengeok" => Ok(Self::ChallengeOk),
            "hash" => Ok(Self::Hash),
            other => Err(ParseError::InvalidHeaderKey(other.to_string())),
        }
    }
}

pub trait RequestKindSpec {
    fn name(&self) -> &'static str;
    fn required_headers(&self) -> &'static [HeaderKind];
}

#[derive(Debug, Clone)]
pub enum RequestKind {
    Send,
    ChallengePlease,
    ChallengeAccepted,
    Certificate,
    HashAuth,
}

impl RequestKindSpec for RequestKind {
    fn name(&self) -> &'static str {
        match self {
            Self::Send => "send",
            Self::ChallengePlease => "challenge please",
            Self::ChallengeAccepted => "challenge accepted",
            Self::Certificate => "cert",
            Self::HashAuth => "hash auth",
        }
    }

    fn required_headers(&self) -> &'static [HeaderKind] {
        use HeaderKind::*;
        match self {
            Self::Send => &[To, Client, Session],
            Self::ChallengePlease => &[Client],
            Self::ChallengeAccepted => &[Session, Client, HMAC],
            Self::Certificate => &[],
            Self::HashAuth => &[Client, Hash],
        }
    }
}

impl RequestKind {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.trim().to_ascii_lowercase().as_str() {
            "send" => Ok(Self::Send),
            "challenge please" => Ok(Self::ChallengePlease),
            "challenge accepted" => Ok(Self::ChallengeAccepted),
            "cert" => Ok(Self::Certificate),
            "hash auth" => Ok(Self::HashAuth),
            other => Err(ParseError::InvalidRequestKind(other.to_string())),
        }
    }
}

pub type Headers = HashMap<HeaderKind, String>;
