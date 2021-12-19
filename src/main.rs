use std::net::TcpStream;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let remote: String = build_remote(&args);

    if let Ok(mut stream) = TcpStream::connect(remote) {
        println!("Connected to the server!");

        const READ_LENGTH: usize = 12;
        let mut buffer = [0; READ_LENGTH];
        let _n = stream.read(&mut buffer[..]);
        println!("received message (bytes): {:?}", &buffer[..READ_LENGTH]);

        let converted_string_buffer = std::str::from_utf8(&buffer).expect("Found invalid UTF-8");
        println!("received message (string): {:?}", converted_string_buffer);
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