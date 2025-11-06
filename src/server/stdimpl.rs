use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::{
    server::db::{DbError, SuitableDB},
    shared::{Request, Response, StatusCode},
};

// ===== database =====
pub struct InMemory {
    users: Arc<Mutex<HashMap<String, String>>>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
    requests: Arc<Mutex<HashMap<String, Vec<Request>>>>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl SuitableDB for InMemory {
    fn store_client(&mut self, user: String, hash: String) {
        self.users.lock().unwrap().insert(user.clone(), hash);
        self.requests.lock().unwrap().entry(user).or_default();
    }

    fn check_client_auth(&self, user: &str, hash: &str) -> bool {
        match self.users.lock().unwrap().get(user) {
            Some(stored) => stored == hash,
            None => false,
        }
    }

    fn store_req_for_user(&self, user: String, req: Request) -> Result<(), DbError> {
        let mut map = self.requests.lock().unwrap();
        if let Some(queue) = map.get_mut(&user) {
            queue.push(req);
            Ok(())
        } else {
            Err(DbError::UserNotFound)
        }
    }

    fn fetch_reqs_for_user(&self, user: &str) -> Option<Vec<Request>> {
        let mut map = self.requests.lock().unwrap();
        map.get_mut(user).map(|queue| std::mem::take(queue))
    }

    fn store_session(&self, user: String, id: String) -> Result<(), DbError> {
        if self.users.lock().unwrap().contains_key(&user) {
            self.sessions.lock().unwrap().insert(user, id);
            Ok(())
        } else {
            Err(DbError::UserNotFound)
        }
    }

    fn get_session(&self, user: &str) -> Option<String> {
        self.sessions.lock().unwrap().get(user).cloned()
    }
}

// ===== server =====
pub struct Server {
    address: std::net::SocketAddr,
    db: InMemory,
}

impl Server {
    pub fn new<T: std::net::ToSocketAddrs>(addr: T) -> Self {
        let mut s = Self {
            address: addr.to_socket_addrs().unwrap().next().unwrap(),
            db: InMemory::new(),
        };
        s.db.store_client(
            "jebediah".into(),
            "9f56e761d79bfdb34304a012586cb04d16b435ef6130091a97702e559260a2f2".into(),
        );
        s
    }

    pub fn listen(mut self) {
        let listener = TcpListener::bind(self.address).unwrap();
        println!("Listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = Vec::new();
                    stream.read_to_end(&mut buf).unwrap();
                    let request_text = String::from_utf8_lossy(&buf);

                    println!("raw request: {}", request_text.to_string());
                    match Request::try_from(request_text.to_string()) {
                        Ok(request) => {
                            println!("got request: {:#?}", request);
                            self.process_request(stream, request);
                        }
                        Err(e) => {
                            let _ = write_error(&mut stream, e.to_status_code(), e.inner());
                        }
                    }
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    fn process_request(&mut self, stream: TcpStream, request: Request) {
        (match request.kind {
            _ => handler_nyi,
        })(stream, request, &mut self.db);
    }
}

fn handler_nyi(mut stream: TcpStream, _req: Request, _db: &mut InMemory) {
    let _ = stream.write_all(
        Response::new(StatusCode::Teapot)
            .header(crate::shared::ResponseHeaderKind::Ok, "true")
            .body("hello! this is an example response")
            .to_string()
            .as_bytes(),
    );
}

fn write_error(
    mut writer: impl Write,
    code: StatusCode,
    message: impl Into<String>,
) -> Result<(), std::io::Error> {
    writer.write_all(Response::with_body(code, message).to_string().as_bytes())
}
