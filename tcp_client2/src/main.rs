use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    match TcpStream::connect("localhost:10120") {
        Ok(stream) => {
            println!("Successfully connected to server in port 10120");

            let stream_clone = Arc::new(Mutex::new(stream));
            let stream_for_read = Arc::clone(&stream_clone);
            let stream_for_write = Arc::clone(&stream_clone);

            let sender = thread::spawn(move || {
                loop {
                    let msg = b"Hello, Server!";
                    let mut stream = stream_for_write.lock().unwrap();
                    stream.write_all(msg).unwrap();
                    drop(stream);
                    println!("Sent message: {}", str::from_utf8(msg).unwrap());
                    thread::sleep(Duration::from_secs(2));
                }
            });

            let receiver = thread::spawn(move || {
                let mut reader = BufReader::new(&*stream_for_read.lock().unwrap());
                loop {
                    let mut response = String::new();
                    if let Ok(_size) = reader.read_line(&mut response) {
                        println!("Response from server: {}", response);
                    } else {
                        eprintln!("Failed to receive response from server");
                    }
                }
            });

            // Wait for both threads to complete
            let _ = sender.join();
            let _ = receiver.join();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}