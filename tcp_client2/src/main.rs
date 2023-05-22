use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:10120") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 10120");

            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "exit" {
                    break;
                }
                
                let msg = input.as_bytes();
                match stream.write_all(msg) {
                    Ok(_) => {
                    },
                    Err(e) => {
                        println!("Failed to send message: {}", e);
                        break;
                    }
                }
                println!("Sent message, awaiting reply...");

                let mut data = [0 as u8; 50]; // using 50 byte buffer for larger messages
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap();
                        println!("Reply: {}", text);
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
