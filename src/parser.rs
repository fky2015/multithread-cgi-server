use std::fs;
use std::io::prelude::*;
use std::ops::*;

extern crate regex;

use regex::Regex;

#[derive(Debug)]
pub struct Pkg {
    pub method: String,
    pub host: String,
    pub user: String,
    pub length: usize,
    pub is_cgi: bool,
}

pub fn parser(s: String) -> Pkg {
    let le = s.len();

    let tokens: Vec<&str> = s.split("\n").collect();

    let status_re = Regex::new(r"^(GET|POST) /(.*)").unwrap();
    let host_re = Regex::new(r"^Host: (.*)").unwrap();
    let user_re = Regex::new(r"^User-Agent: (.*)").unwrap();
    let cgi_re = Regex::new(r"(.*)/cgi-bin(.*)").unwrap();

    let mut method = String::new();
    let mut host = String::new();
    let mut user = String::new();
    let mut is_cgi = false;

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
                is_cgi = true;
                //        println!("user : {}", user)
            }
            None => {}
        }

        //    println!("length : {}", le);
    }

    Pkg {
        method: method.clone(),
        host: host.clone(),
        user: user.clone(),
        length: le,
        is_cgi,
    }
}

