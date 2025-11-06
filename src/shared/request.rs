use crate::shared::RequestKindSpec;
use std::collections::HashMap;

use crate::shared::{HeaderKind, ParseError, RequestKind};

#[derive(Debug)]
pub struct Request {
    pub version: String,
    pub kind: RequestKind,
    pub headers: HashMap<HeaderKind, String>,
    pub body: Option<String>,
}

impl TryFrom<String> for Request {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, ParseError> {
        let mut lines = value.lines();

        // headline
        let first_line = lines
            .next()
            .ok_or_else(|| ParseError::InvalidFormat("missing request kind".to_string()))?;

        let (version, kind) = if let Some((version_str, kind_str)) = first_line.split_once(" ") {
            let kind = RequestKind::from_str(kind_str)
                .map_err(|_| ParseError::InvalidRequestKind(kind_str.to_string()))?;
            (version_str.to_string(), kind)
        } else {
            return Err(ParseError::InvalidFormat("invalid headline".into()));
        };

        // headers
        let mut headers = HashMap::new();
        for line in &mut lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                break;
            }

            // expect "key: value"
            let (key, value) = trimmed.split_once(':').ok_or_else(|| {
                ParseError::InvalidFormat(format!("\"{trimmed}\" is not a valid header line. headers must be formatted as [name]: [value]"))
            })?;

            let key_kind = HeaderKind::from_str(key)?;
            let value = value.trim().to_string();
            if value.is_empty() {
                return Err(ParseError::HeaderEmpty(format!("header empty: {key}")));
            };

            headers.insert(key_kind, value.trim().to_string());
        }
        for required in kind.required_headers() {
            if !headers.contains_key(&required) {
                return Err(ParseError::HeaderMissing(format!("{required:?}")));
            }
        }

        // body
        let body_text: String = lines.collect::<Vec<_>>().join("\n").trim().to_string();
        let body = if !body_text.is_empty() {
            Some(body_text)
        } else {
            None
        };

        Ok(Request {
            kind,
            headers,
            body: body,
            version,
        })
    }
}
