use std::{collections::HashMap, fmt::Display};

use crate::shared::{ResponseHeaderKind, StatusCode};

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    headers: HashMap<ResponseHeaderKind, String>,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: None,
        }
    }
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self.header(ResponseHeaderKind::Length, body.len().to_string())
    }
    pub fn with_body(status: StatusCode, body: impl Into<String>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Some(body.into()),
        }
    }
    pub fn header(mut self, kind: ResponseHeaderKind, value: impl Into<String>) -> Self {
        self.headers.insert(kind, value.into());
        self
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        self.to_string().as_bytes().into()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // headline
        writeln!(
            f,
            "{} status {}: {}", // the first line could be either "v0.1 5" or "v0.1 status 5:
            // offline messages"
            crate::VERSION,
            self.status.clone() as i32,
            self.status
        )?;

        // headers
        for (key, value) in &self.headers {
            writeln!(f, "{}: {}", key, value)?;
        }
        // body must be separated by a newline
        if let Some(body) = &self.body {
            writeln!(f)?;
            writeln!(f, "{}", body)?;
        }

        Ok(())
    }
}
