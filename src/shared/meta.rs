// struct definition macros
macro_rules! parse_errors {
    ($struct_name:ident is $(
        $variant:ident => $status:ident
    ),* $(,)?) => {
        use std::fmt::Display;

        #[derive(Debug, Clone)]
        pub enum $struct_name {
            $($variant(String)),*
        }

        impl $struct_name {
            pub fn to_status_code(&self) -> StatusCode {
                match self {
                    $($struct_name::$variant(_) => StatusCode::$status),*
                }
            }

            pub fn inner(&self) -> &str {
                match self {
                    $($struct_name::$variant(s) => s),*
                }
            }
        }
    };
}

macro_rules! request_kinds {
    ($struct_name:ident is $(
        $variant:ident = {
            name: $name:literal,
            required: [$($header:ident),* $(,)?] // required headers
            $(, optional: [$($optional_header:ident),* $(,)?] )? // doesnt do anything lol
            $(, possible_responses: [$($response:ident),* $(,)?] )? // doesnt do anything either
        }
    ),* $(,)?) => {
        use HeaderKind::*;

        #[derive(Debug, Clone)]
        pub enum $struct_name {
            $($variant),*
        }
        // request kinds
        pub trait RequestKindSpec {
            fn name(&self) -> &'static str;
            fn required_headers(&self) -> &'static [HeaderKind];
            fn optional_headers(&self) -> &'static [HeaderKind];
            fn possible_responses(&self) -> &'static [ResponseKind];
        }

        impl RequestKindSpec for $struct_name {
            fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $name),*
                }
            }

            fn required_headers(&self) -> &'static [HeaderKind] {
                match self {
                    $(Self::$variant => &[$($header),*]),*
                }
            }
            fn optional_headers(&self) -> &'static [HeaderKind] {
                match self {
                    $(Self::$variant => &[$($($optional_header),*)?]),*
                }
            }

            fn possible_responses(&self) -> &'static [ResponseKind] {
                use ResponseKind::*;
                match self {
                    $(Self::$variant => &[$($($response),*)?]),*
                }
            }
        }

        impl $struct_name {
            pub fn from_str(s: &str) -> Result<Self, ParseError> {
                match s.trim().to_ascii_lowercase().as_str() {
                    $($name => Ok(Self::$variant),)*
                    other => Err(ParseError::InvalidRequestKind(other.to_string())),
                }
            }
        }
    };
}

// status codes
macro_rules! status_codes {
    ($struct_name:ident is $($name:ident = $code:literal $lexeme:literal),* $(,)?) => {

        #[derive(Clone, Debug)]
        pub enum $struct_name {
            $($name = $code),*
        }

        impl Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $($struct_name::$name => $lexeme),*
                    })
                }
        }
        impl From<ParseError> for $struct_name {
            fn from(err: ParseError) -> Self {
                err.to_status_code()
            }
        }
    };
}
macro_rules! headers {
    ($struct_name:ident is $($name:ident = $lexeme:literal),* $(,)?) => {

        #[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
        pub enum $struct_name {
            $($name),*
        }

        impl Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $($struct_name::$name => $lexeme),*
                    })
                }
        }
        impl $struct_name {
            pub fn from_str(s: &str) -> Result<Self, ParseError> {
                match s.to_ascii_lowercase().as_str() {
                    $($lexeme => Ok(Self::$name),)*
                    other => Err(ParseError::InvalidHeaderKey(other.to_string())),
                }
            }
        }

    };
}

macro_rules! response_kinds {
    ($struct_name:ident is $(
        $variant:ident = {
            code: $code:ident,
            required: [$($header:ident),* $(,)?],
            body: $body_req:ident
        }
    ),* $(,)?) => {
        #[derive(Debug, Clone)]
        pub enum $struct_name {
            $($variant),*
        }

        impl $struct_name {
            /// numeric status code
            pub fn code(&self) -> StatusCode {
                match self {
                    $(Self::$variant => StatusCode::$code),*
                }
            }

            pub fn required_headers(&self) -> &'static [ResponseHeaderKind] {
                use ResponseHeaderKind::*;
                match self {
                    $(Self::$variant => &[$($header),*]),*
                }
            }

            /// whether the response must have a body, must not, or may optionally include one
            pub fn body_requirement(&self) -> BodyRequirement {
                match self {
                    $(Self::$variant => BodyRequirement::$body_req),*
                }
            }

            /// parses from numeric code
            pub fn from_status(code: StatusCode) -> Result<Self, ParseError> {
                match code {
                    $(StatusCode::$code => Ok(Self::$variant),)*
                    _ => Err(ParseError::InvalidRequestKind(code.to_string())),
                }
            }
        }
    };
}

pub(crate) use headers;
pub(crate) use parse_errors;
pub(crate) use request_kinds;
pub(crate) use response_kinds;
pub(crate) use status_codes;
