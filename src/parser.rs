use std::fs;
use std::io::prelude::*;
use std::ops::*;

extern crate regex;
use regex::Regex;

struct pkg {
    method : String,
    host : String,
    user : String,
    length : usize,
    iscgi : bool
}

fn parser (s : String) -> pkg {
    let le = s.len();

    let tokens:Vec<&str>= s.split("\n").collect();
    
    let status_re = Regex::new(r"^(GET|POST) /(.*)").unwrap();
    let host_re = Regex::new(r"^Host: (.*)").unwrap(); 
    let user_re = Regex::new(r"^User-Agent: (.*)").unwrap();
    let cgi_re = Regex::new(r"(.*)/cgi-bin(.*)").unwrap();

    let mut method = String::new();
    let mut host = String::new();
    let mut user = String::new();
    let mut iscgi = false;

    for i in &tokens {
    //    println!("{}", i);

        match status_re.captures(i) {
        Some(cap) => {
            method = cap.index(1).to_string();
    //        println!("METHOD : {}", method)
        }
        None => {}
        }

        match host_re.captures(i) {
        Some(cap) => {
            host = cap.index(1).to_string();
    //        println!("host : {}", host)
        }
        None => {}
        }

        match user_re.captures(i) {
        Some(cap) => {
            user = cap.index(1).to_string();
    //        println!("user : {}", user)
        }
        None => {}
        }

        match cgi_re.captures(i) {
        Some(cap) => {
            iscgi = true;
    //        println!("user : {}", user)
        }
        None => {}
        }

    //    println!("length : {}", le);

    }
    let pkg1 = pkg {
            method: method.clone(),
            host: host.clone(),
            user: user.clone(),
            length: le,
            iscgi: iscgi
        };

    return pkg1;
}

// fn main() {

//     let s = "GET /cgi-bin/xx HTTP/1.1\nHost: localhost:8000\nConnection: keep-alive\nCache-Control: max-age=0\nUpgrade-Insecure-Requests: 1\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.75 Safari/537.36\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9\nSec-Fetch-Site: cross-site\nSec-Fetch-Mode: navigate\nSec-Fetch-User: ?1\nSec-Fetch-Dest: document\nAccept-Encoding: gzip, deflate, br\nAccept-Language: zh-CN,zh;q=0.9,en;q=0.8\nCookie: _ga=GA1.1.2137995243.1551714284; csrftoken=aJ957eGVTpa9vY0y4vio7MpE1mlPKBGwGgYD7EoPcACTmtJWxOenepIN9epB08RX; p_h5_u=62F235C2-C6A4-4E1F-9F3A-FE6ADE9AAEFE; __atuvc=0%7C35%2C0%7C36%2C29%7C37%2C14%7C38%2C1%7C39";
    
//     let pkgp = parser(s.to_string());

//     println!("method : {}", pkgp.method);
//     println!("host : {}", pkgp.host);
//     println!("user : {}", pkgp.user);
//     println!("length : {}", pkgp.length);
//     println!("iscgi : {}", pkgp.iscgi);

// }
