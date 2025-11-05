use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use sha2::{Digest, Sha256};

use crate::{
    comms::{Request, Response},
    encryption::gen_token,
    meta::{HeaderKind, StatusCode},
};

pub struct Server {
    address: std::net::SocketAddr,
}

fn handler_nyi(mut stream: TcpStream, req: Request) {
    let _ = stream.write_all(
        Response::new(StatusCode::Unsupported)
            .to_string()
            .as_bytes(),
    );
}

fn handler_hash(mut stream: TcpStream, req: Request) {
    let hash = req.headers.get(&HeaderKind::Hash).unwrap();
    let client = req.headers.get(&HeaderKind::Client).unwrap();

    let user_hash_mock =
        String::from("9f56e761d79bfdb34304a012586cb04d16b435ef6130091a97702e559260a2f2"); // get from db
    let session_id_mock = gen_token(8); // store in db, maybe replace with jwt later?
    // todo use constant_time_eq
    if *hash == user_hash_mock {
        let _ = stream.write_all(
            Response::new(StatusCode::HashAccepted)
                .to_string()
                .as_bytes(),
        );
    } else {
        let _ = stream.write_all(
            Response::new(StatusCode::AuthInvalid)
                .to_string()
                .as_bytes(),
        );
    }
}

type Handler = fn(TcpStream, Request);

fn process_request(mut stream: TcpStream, request: Request) {
    let handler: Handler = match request.kind {
        crate::meta::RequestKind::Send => handler_nyi,
        crate::meta::RequestKind::ChallengePlease => handler_nyi,
        crate::meta::RequestKind::ChallengeAccepted => handler_nyi,
        crate::meta::RequestKind::Certificate => handler_nyi,
        crate::meta::RequestKind::HashAuth => handler_hash,
    };
    handler(stream, request);
}

fn write_error(
    mut writer: impl Write,
    code: StatusCode,
    message: impl Into<String>,
) -> Result<(), std::io::Error> {
    writer.write_all(Response::with_body(code, message).to_string().as_bytes())
}

impl Server {
    pub fn new<T: std::net::ToSocketAddrs>(addr: T) -> Self {
        Self {
            address: addr.to_socket_addrs().unwrap().next().unwrap(),
        }
    }
    pub fn listen(self) {
        let listener = TcpListener::bind(self.address).unwrap();
        loop {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).unwrap();

            let request_text = String::from_utf8_lossy(&buf);
            match Request::try_from(request_text.to_string()) {
                Ok(request) => {
                    println!("got request: {:#?}", request);
                    process_request(stream, request);
                }
                Err(e) => {
                    write_error(stream, e.clone().to_status_code(), e.inner()).unwrap();
                }
            }
        }
    }
}
