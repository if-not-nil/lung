pub mod stdimpl;
use rand::{distr::Alphanumeric, prelude::*};
pub use stdimpl::Server;
pub mod db;

/// generate a session token or user id. the standard is 16 chars
pub fn gen_token(n: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
