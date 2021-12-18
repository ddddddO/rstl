use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    if let Ok(_stream) = TcpStream::connect("127.0.0.1:1123") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
