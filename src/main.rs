use ctrlc;
use dotenv;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::spawn;

mod cgi;
mod filereader;
mod parser;
mod thread_pool;

enum LoggingSignal {
    Logging(String),
    Shutdown,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".into());
    let logfile = env::var("LOG_FILE").unwrap_or_else(|_| "log/logfile.txt".into());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    let pool = Arc::new(Mutex::new(thread_pool::ThreadPool::new(10)?));

    let pool_handler = pool.clone();

    // (Almost) Gracefully exit.

    let (log_sender, log_receiver) = mpsc::channel();

    let t = spawn(move || {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(logfile)
            .unwrap();

        loop {
            match log_receiver.recv() {
                Ok(LoggingSignal::Logging(message)) => {
                    //println!("{}", message);
                    file.write(message.as_bytes());
                }
                Ok(LoggingSignal::Shutdown) => {
                    println!("Logger exits, close logfile handler!");
                    break;
                }
                Err(e) => {
                    println!("Logger exits");
                    println!("{:?}", e);
                }
            }
        }
    });

    let mut t = Some(Some(t));
    let log_sender_shutdown = log_sender.clone();
    ctrlc::set_handler(move || {
        let mut pool = pool_handler.lock().unwrap();
        pool.manual_drop();
        let t = t.replace(None);
        match t {
            Some(Some(t)) => {
                log_sender_shutdown.send(LoggingSignal::Shutdown);
                t.join();
            }
            _ => {
                println!("Logger thread have been destroyed!");
            }
        };
        exit(0);
    })
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let pool = pool.lock().unwrap();
        let log_sender = log_sender.clone();
        pool.execute(move || {
            // std::thread::sleep(std::time::Duration::from_millis(100));

            let log = handle_connection(stream);
            log_sender.send(LoggingSignal::Logging(log));
            std::mem::drop(log_sender);
        });
    }

    println!("Won't execute here.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> String {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    let b = parser::parser(&buffer);
    if b.is_err() {
        let status_line = "HTTP/1.1 404 Not Found";
        let response = format!("{}", status_line);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        let time = chrono::Local::now().to_rfc3339();

        return format!(
            "{} {}\n",
            time,
            "empty-request"
        );
    }

    let b = b.unwrap();
    let response =
        if b.iscgi {
            let result = if b.method == "GET" {
                cgi::cgi_caller_get(&b.path, &b.query_string)
            } else {
                cgi::cgi_caller_post(&b.path, &b.content_length, &b.content_type, &b.body_string)
            };

            match result {
                Ok(content) => {
                    let status_line = "HTTP/1.1 200 OK";
                    format!("{}\r\n{}", status_line, content)
                }
                _ => {
                    let status_line = "HTTP/1.1 404 Not Found";
                    format!("{}", status_line)
                }
            }
        } else {
            let res = filereader::readfile(b.path.clone());

            match res {
                Some(res) => {
                    let status_line = "HTTP/1.1 200 OK";
                    let content_type = res.1;
                    let content = res.0;
                    format!("{}\r\n{}\r\n\r\n{}", status_line, content_type, content)
                }
                _ => {
                    let status_line = "HTTP/1.1 404 Not Found";
                    format!("{}", status_line)
                }
            }
        };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    let time = chrono::Local::now().to_rfc3339();
    format!(
        "{} {} {} {} {} {} {}\n",
        time, b.host, b.method, b.user, b.url, b.path, b.query_string
    )
}
