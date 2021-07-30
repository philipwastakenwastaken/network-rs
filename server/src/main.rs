mod server;

use server::Server;
use common::transaction::Transaction;
use common::constants::LISTEN_ADDR;

fn main() {
    println!("Hello, server!");
    let _tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 42.0);
    println!("Created tx...");
    println!("From: {:?}", _tx.from());
    println!("To: {:?}", _tx.to());
    println!("Amount: {:?}", _tx.amount());


    let server = Server::bind(LISTEN_ADDR, "./private.pem", "./server_key.pem");
    server.listen();
}
