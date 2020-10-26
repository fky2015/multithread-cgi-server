use std::fs::{metadata, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn readfile(filepath: String) -> Option<(String, String)> {
    let mut content = String::new();
    let mut pathbuf = PathBuf::from("");
    pathbuf.push("static");
    let mut path = filepath.as_str();
    if path.chars().nth(0) == Some('/') || path.chars().nth(0) == Some('\\') {
        path = &path[1..];
    }
    pathbuf.push(path);
    let pathstr = pathbuf.as_path().display().to_string();

    match metadata(pathstr) {
        Ok(f) => {
            if f.is_dir() {
                pathbuf.push("index.html")
            }
        }
        _ => {
            return None;
        }
    }

    let pathstr = pathbuf.as_path().display().to_string();

    // pathstr = (&pathstr[1..]).to_string();
    // println!("{}", pathstr);

    let tokens: Vec<&str> = filepath.split(".").collect();
    let extender = tokens[tokens.len() - 1];
    let content_type = match extender {
        "html" => "text/html",
        "xml" => "text/xml",
        "css" => "text/css",
        "gif" => "image/gif",
        "jpg" => "image/jpg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "ico" => "application/x-ico",
        _ => "text/html",
    }
    .to_string();

    match File::open(pathstr) {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
            Some((content, content_type))
        }
        Err(_) => None,
    }
}

#[test]
fn file_index() {
    let res = readfile("/".to_string());
}

#[test]
fn file_test1() {
    let res = readfile("/index.html".to_string());
}

#[test]
fn file_tes21() {
    let res = readfile("index.html".to_string());
}

#[test]
fn file_test3() {
    let res = readfile("index1.html".to_string());
}
