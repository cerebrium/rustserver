use std::net::TcpListener;

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
                Ok((stream, address)) => {
                    println!("Connection from {}", address);
                    // spawn a new thread for each connection

                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}