use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;

// holds the data for the server struct
pub struct Server {
    addr: String,
}

// Functionality for the struct
impl Server {
    // needs self to be able to access the data
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // convert recoverable eror to unrecoverable via unwrap
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{} request read from {}", String::from_utf8_lossy(&buffer), address);

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                },
                                Err(e) => {
                                    println!("Error parsing request: {}", e);
                                }
                                }
                            },
                        Err(e) => {
                            println!("Error reading from buffer: {}", e);
                        }
                    }
                },
                    
                Err(e) => {
                    println!("Error creating connection: {}", e);
                }
            }
        }
    }
}