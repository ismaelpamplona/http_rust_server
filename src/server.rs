use crate::http::{Request, Response, StatusCode};
use std::{
    convert::TryFrom,
    io::{Read, Write},
    net::TcpListener,
};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let res = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                    Response::new(StatusCode::Ok, Some("Test!".to_string()))
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = res.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Reading from the connection failed: {}", e),
                    }
                }
                Err(e) => println!("The attempt to establish a connection has failed: {}", e),
            }
        }
    }
}
