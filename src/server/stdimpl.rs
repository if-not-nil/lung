use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    server::db::{self, SuitableDB},
    shared::{Request, RequestKind, Response, StatusCode},
};

fn handler_nyi(mut stream: TcpStream, _req: Request, _db: &mut db::InMemory) {
    let _ = stream.write_all(
        Response::new(StatusCode::Unsupported)
            .to_string()
            .as_bytes(),
    );
}

type Handler = fn(TcpStream, Request, &mut db::InMemory);

fn write_error(
    mut writer: impl Write,
    code: StatusCode,
    message: impl Into<String>,
) -> Result<(), std::io::Error> {
    writer.write_all(Response::with_body(code, message).to_string().as_bytes())
}

pub struct Server {
    address: std::net::SocketAddr,
    db: db::InMemory, // can be any db really, depending on the implementation
}

impl Server {
    pub fn new<T: std::net::ToSocketAddrs>(addr: T) -> Self {
        let mut s = Self {
            db: db::InMemory::new(),
            address: addr.to_socket_addrs().unwrap().next().unwrap(),
        };
        s.db.store_client(
            "jebediah".to_string(),
            "9f56e761d79bfdb34304a012586cb04d16b435ef6130091a97702e559260a2f2".to_string(),
        );
        s
    }
    pub fn listen(mut self) {
        let listener = TcpListener::bind(self.address).unwrap();
        loop {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).unwrap();

            let request_text = String::from_utf8_lossy(&buf);
            match Request::try_from(request_text.to_string()) {
                Ok(request) => {
                    println!("got request: {:#?}", request);
                    self.process_request(stream, request);
                }
                Err(e) => {
                    write_error(stream, e.clone().to_status_code(), e.inner()).unwrap();
                }
            }
        }
    }

    fn process_request(&mut self, stream: TcpStream, request: Request) {
        let handler: Handler = match request.kind {
            _ => handler_nyi,
        };
        handler(stream, request, &mut self.db);
    }
}
