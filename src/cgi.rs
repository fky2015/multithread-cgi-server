use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::path::PathBuf;

use crate::cgi::CgiCallError::{RuntimeError, FileNotExists};

const CGI_ROOT_PATH: &str = "./cgi-part";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CgiCallError {
    FileNotExists,
    RuntimeError,
}

/*
 Function for get,
 with cgi bin path and query string,
 cgi path must be a relative path,
 see get_test in the bottom for example
 */
pub fn cgi_caller_get(
    cgi_path: &str,
    query_string: &str,
) -> Result<String, CgiCallError> {
    let real_path: PathBuf = [CGI_ROOT_PATH, cgi_path].iter().collect();
    if !real_path.exists() {
        return Err(FileNotExists);
    }

    let output = Command::new(real_path.to_str().unwrap())
        .env("REQUEST_METHOD", "GET")
        .env("QUERY_STRING", query_string)
        .output().unwrap();

    match output.status.success() {
        true => {
            let re = String::from_utf8(output.stdout).unwrap();
            Ok(re)
        }
        false => Err(RuntimeError),
    }
}
/*
 Function for post,
 with cgi bin path, content length, conteng type and request body as a str
 cgi path must be a relative path,
 see post_test in the bottom for example
 */

pub fn cgi_caller_post(
    cgi_path: &str,
    content_length: &str,
    content_type: &str,
    body_string: &str,
) -> Result<String, CgiCallError> {
    let real_path: PathBuf = [CGI_ROOT_PATH, cgi_path].iter().collect();
    if !real_path.exists() {
        return Err(FileNotExists);
    }

    let mut child = Command::new(real_path.to_str().unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .env("REQUEST_METHOD", "POST")
        .env("CONTENT_LENGTH", content_length)
        .env("CONTENT_TYPE", content_type)
        .spawn().unwrap();

    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(body_string.as_ref()).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    let re = String::from_utf8(output.stdout).unwrap();

    match output.status.success() {
        true => {
            Ok(re)
        }
        false => Err(RuntimeError),
    }
}


fn cgi_file_exists(cgi_path: &str) -> bool {
    use std::path::Path;
    Path::new(cgi_path).exists()
}


#[cfg(test)]
mod cgi_tests {
    use crate::cgi::{cgi_caller_get, cgi_caller_post, cgi_file_exists, CgiCallError};


    #[test]
    fn get_test1() {
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/calculator.py";
        let result = cgi_caller_get(path, query_string).unwrap_or("".to_string());
        assert_eq!("Content-type:text/html\n\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>两数之和与之积</title>\n</head>\n<body>\n<h2>两数之和: 357</h2>\n<h2>两数之积: 28782</h2>\n</body>\n</html>\n".to_string(),
                   result);
    }

    #[test]
    fn get_test2() {
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/not-exists";
        let result = cgi_caller_get(path, query_string).err().unwrap();
        assert_eq!(CgiCallError::FileNotExists, result);
    }

    #[test]
    fn get_test3() {
        let query_string = "xxx";
        let path = "cgi-bin/calculator.py";
        let result = cgi_caller_get(path, query_string).err().unwrap();
        assert_eq!(CgiCallError::RuntimeError, result);
    }

    #[test]
    fn post_test1() {
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/calculator.py";
        let content_length = query_string.len().to_string();
        let result = cgi_caller_post(path,
                                     content_length.as_str(),
                                     "application/x-www-form-urlencoded",
                                     query_string).unwrap();
        assert_eq!("Content-type:text/html\n\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>两数之和与之积</title>\n</head>\n<body>\n<h2>两数之和: 357</h2>\n<h2>两数之积: 28782</h2>\n</body>\n</html>\n".to_string(),
                   result);
    }

    #[test]
    fn post_test2() {
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/not-exists";
        let content_length = query_string.len().to_string();
        let result = cgi_caller_post(path,
                                     content_length.as_str(),
                                     "application/x-www-form-urlencoded",
                                     query_string).err().unwrap();
        assert_eq!(CgiCallError::FileNotExists, result);
    }

    #[test]
    fn post_test3() {
        let query_string = "xxx";
        let path = "cgi-bin/calculator.py";
        let content_length = query_string.len().to_string();
        let result = cgi_caller_post(path,
                                     content_length.as_str(),
                                     "application/x-www-form-urlencoded",
                                     query_string).err().unwrap();
        assert_eq!(CgiCallError::RuntimeError, result);
    }

    #[test]
    fn post_test4() {
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/calculator.py";
        let content_length = query_string.len().to_string();
        let result = cgi_caller_post(path,
                                     content_length.as_str(),
                                     "WhatFType",
                                     query_string).err().unwrap();
        assert_eq!(CgiCallError::RuntimeError, result);
    }
}

