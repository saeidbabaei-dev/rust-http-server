use std::{io, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self { listener })
    }

    pub fn run(&self) -> io::Result<()> {
        println!("Listening on {}", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        println!(
                            "Worker {:?}",
                            thread::current().id()
                        );

                        Server::handle_connection(stream);
                    });
                },
                Err(error) => {
                    eprintln!("Failed to accept connection: {}", error);
                }
            }
        }

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) {
        println!(
    "Running on thread {:?}",
    thread::current().id()
);
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("Failed to set read timeout");

        println!("New connection from {}", stream.peer_addr().unwrap());

        let mut buffer = [0u8; 4096];

        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client closed the connection.");
                return;
            }
            Ok(bytes_read) => {
                println!("Received {} bytes", bytes_read);

                let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                println!("---------------- REQUEST ----------------");
                println!("{}", request);
                println!("-----------------------------------------");

                let body = "Hello, World!";

                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: {}\r\n\
                    Connection: close\r\n\
                    \r\n\
                    {}",
                    body.len(),
                    body
                );
                if let Err(error) = stream.write_all(response.as_bytes()) {
                    eprintln!("Write error: {}", error);
                    return;
                }
                if let Err(error) = stream.flush() {
                    eprintln!("Flush error: {}", error);
                }
            }

            Err(error) => {
                eprintln!("Failed to read: {}", error);
            }
        }
    }
}
