use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::shared::Request;


pub struct InMemory {
    users: Arc<Mutex<HashMap<String, String>>>,
    requests: Arc<Mutex<HashMap<String, Vec<Request>>>>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub enum DbError {
    UserNotFound,
}

impl SuitableDB for InMemory {
    fn store_client(&mut self, user: String, hash: String) {
        let mut store = self.users.lock().unwrap();
        store.insert(user, hash);
        store
            .iter()
            .for_each(|(k, v)| println!("store is {}: {}\n", k, v));
    }

    fn check_client_auth(&self, user: &String, hash: &String) -> bool {
        let store = self.users.lock().unwrap();
        println!("provided {} for {}", hash, user);
        let record = store.get(user);
        println!("tried to match with {:#?}", record);
        if let Some(dbhash) = record {
            dbhash == hash
        } else {
            false
        }
    }

    fn store_req_for_user(&self, user: String, req: Request) -> Result<(), DbError> {
        let mut map = self.requests.lock().unwrap();
        match map.get_mut(&user) {
            Some(requests) => {
                requests.push(req);
                Ok(())
            }
            None => Err(DbError::UserNotFound),
        }
    }

    fn store_session(&self, user: String, id: String) -> Result<(), DbError> {
        todo!()
    }
}

pub trait SuitableDB {
    fn store_client(&mut self, user: String, hash: String);
    fn check_client_auth(&self, user: &String, hash: &String) -> bool;
    fn store_req_for_user(&self, user: String, req: Request) -> Result<(), DbError>;
    fn store_session(&self, user: String, id: String) -> Result<(), DbError>;
}
