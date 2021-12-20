use std::net::TcpStream;
use std::env;
use std::io::Read;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let remote: String = build_remote(&args);

    if let Ok(mut stream) = TcpStream::connect(remote) {
        println!("Connected to the server!");

        const READ_LENGTH: usize = 12;
        let mut buffer = [0; READ_LENGTH];

        // refs:
        // http://srgia.com/docs/rfc854j.html [TELNET 命令構造]
        // https://stackoverflow.com/questions/10413963/telnet-iac-command-answering
        // http://www.iana.org/assignments/telnet-options/telnet-options.xhtml

        // client <- remote
        let _n = stream.read(&mut buffer[..]);
        println!("received message (bytes): {:?}", &buffer[..READ_LENGTH]); // [255, 253, 24, 255, 253, 32, 255, 253, 35, 255, 253, 39]

        let send_bytes = [255, 252, 24, 255, 252, 32, 255, 252, 35, 255, 252, 39];
        // client -> remote
        let _n = stream.write(&send_bytes[..]);

        // client <- remote
        let _n = stream.read(&mut buffer[..]);
        println!("received message2 (bytes): {:?}", &buffer[..READ_LENGTH]); // [255, 251, 3, 255, 253, 1, 255, 253, 31, 255, 251, 5]

        let send_bytes_2 = [255, 254, 3, 255, 254, 1, 255, 254, 31, 255, 254, 5];
        // client -> remote
        let _n = stream.write(&send_bytes_2[..]);

        // client <- remote
        let _n = stream.read(&mut buffer[..]);
        println!("received message3 (bytes): {:?}", &buffer[..READ_LENGTH]); // [255, 253, 33, 255, 253, 1, 255, 253, 31, 255, 251, 5]
    } else {
        println!("Couldn't connect to server...");
    }
}

fn build_remote(args: &Vec<String>) -> String {
    let remote = if args.len() == 3 {
        let host = &args[1];
        let port = &args[2];

        format!("{}:{}", host, port)
    } else if args.len() == 2 {
        let host = &args[1];
        let default_port = "23".to_string();

        format!("{}:{}", host, default_port)
    } else {
        // TODO: error handring
        "args error".to_string()
    };

    remote
}