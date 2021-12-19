use std::net::TcpStream;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let remote: String = build_remote(&args);

    if let Ok(_stream) = TcpStream::connect(remote) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}

fn build_remote(args: &Vec<String>) -> String {
    if args.len() == 3 {
        let host = &args[1];
        let port = &args[2];
        return format!("{}:{}", host, port);
    } else if args.len() == 2 {
        let host = &args[1];
        let default_port = "23".to_string();
        return format!("{}:{}", host, default_port);
    } else {
        // TODO: error handring
        return "args error".to_string()
    }
}