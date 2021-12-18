use std::net::TcpStream;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut remote: String = "".to_string();

    if args.len() == 3 {
        let host = &args[1];
        let port = &args[2];
        remote = format!("{}:{}", host, port);
    } else if args.len() == 2 {
        let host = &args[1];
        let default_port = "23".to_string();
        remote = format!("{}:{}", host, default_port);
    } else {
        println!("args error");
    }

    if let Ok(_stream) = TcpStream::connect(remote) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
