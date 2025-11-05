use protocol::Server;

fn main() {
    Server::new("0.0.0.0:1337").listen();
}
