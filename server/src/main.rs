mod server;

use server::{Server, LISTEN_ADDR};

fn main() {
    println!("Hello, world!");
    let server = Server::bind(LISTEN_ADDR);
    server.listen();
}
