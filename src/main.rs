use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{self};
use std::env;

const PORT : &str = "PORT";

fn main() {

    let port : String = match env::var(PORT) {
        Ok(input_port) => input_port,
        Err(err) => {
            println!("error getting port : {} defaulting to 4444", err.to_string());
            "4444".to_string()
        }
    };

    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("failed to write to stream");

    let mut buffer = String::new();
    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", buffer);
                stream.write((&buffer).as_ref()).expect("failed to write to stream");
                buffer.clear()
            }
            Err(error) => println!("error: {}", error),
        }
    }
}