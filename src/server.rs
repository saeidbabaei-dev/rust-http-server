use crate::connection::Connection;
use std::net::TcpListener;
use std::time::Duration;
use std::{io, thread};

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
                        println!("Worker {:?}", thread::current().id());

                        Server::handle_connection(stream);
                    });
                }
                Err(error) => {
                    eprintln!("Failed to accept connection: {}", error);
                }
            }
        }

        Ok(())
    }

    fn handle_connection(stream: std::net::TcpStream) {
        let mut connection = Connection::new(stream);

        connection.set_read_timeout(Duration::from_secs(5)).unwrap();

        println!("Client {}", connection.peer_addr().unwrap());

        let mut buffer = [0u8; 4096];

        match connection.read(&mut buffer) {
            Ok(0) => {
                println!("Connection closed");
                return;
            }

            Ok(bytes_read) => {
                println!("Read {} bytes", bytes_read);

                let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                println!("{}", request);

                let body = "Hello, World!";

                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Length: {}\r\n\
                    Content-Type: text/plain\r\n\
                    Connection: close\r\n\
                    \r\n\
                    {}",
                    body.len(),
                    body
                );

                connection.write_all(response.as_bytes()).unwrap();

                connection.flush().unwrap();
            }

            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
