use dotenv;
use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use multithread_cgi_server::ThreadPool;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("HOST").unwrap_or_else(|_| "8000".into());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    let pool = ThreadPool::new(4)?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    // TODO: parse buffer to get file

    // TODO: handle read file or 404

    // ok
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    // or not found
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    stream
        .write(format!("{}{}", status_line, "").as_bytes())
        .unwrap();
    stream.flush().unwrap();

    // TODO: logging
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
