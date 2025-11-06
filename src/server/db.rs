use crate::shared::Request;

#[derive(Debug)]
pub enum DbError {
    UserNotFound,
    SessionNotFound,
}

pub trait SuitableDB: Send + Sync {
    fn store_client(&mut self, user: String, hash: String);
    fn check_client_auth(&self, user: &str, hash: &str) -> bool;
    fn store_req_for_user(&self, user: String, req: Request) -> Result<(), DbError>;
    fn fetch_reqs_for_user(&self, user: &str) -> Option<Vec<Request>>;
    fn store_session(&self, user: String, id: String) -> Result<(), DbError>;
    fn get_session(&self, user: &str) -> Option<String>;
}
