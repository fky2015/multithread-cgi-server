use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn readfile(filepath: String) -> (String, String) {
    let mut content = String::new();
    let mut pathbuf = PathBuf::from("");
    pathbuf.push("static");
    let mut path = filepath.as_str();
    if path.chars().nth(0) == Some('/') || path.chars().nth(0) == Some('\\') {
        path = &path[1..];
    }
    pathbuf.push(path);
    let mut pathstr = pathbuf.as_path().display().to_string();
    // pathstr = (&pathstr[1..]).to_string();
    // println!("{}", pathstr);

    let tokens:Vec<&str> = filepath.split(".").collect();
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
        _ => "text/html"
    }.to_string();
    // println!("content-type : {}", content_type);

    let mut file = match File::open(pathstr) {
        Ok(mut file) => {
            println!("ok");
            file.read_to_string(&mut content).unwrap();
        },
        Err(_) => {
            content = "<!DOCTYPE html><html>
            <head><title>File not found - 404</title></head>
            <body><h3>Sorry, the file you were looking for was not found - 404</h3></body></html>".to_string();
        }
    };
    (content, content_type)
}

    #[test]
    fn file_test1() {
        let res = readfile("/index.html".to_string());
        println!("{}\n{}\n", res.0, res.1);
    }

    #[test]
    fn file_tes21() {
        let res = readfile("index.html".to_string());
        println!("{}\n{}\n", res.0, res.1);
    }

    #[test]
    fn file_test3() {
        let res = readfile("index1.html".to_string());
        println!("{}\n{}\n", res.0, res.1);
    }