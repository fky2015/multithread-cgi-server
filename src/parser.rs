
extern crate httparse;
use httparse::Request;

#[derive(Debug)]
pub struct Pkg {
    pub method : String,
    pub host : String,
    pub user : String,
    pub url : String,
    pub path : String,
    pub query_string : String,
    pub content_length : String,
    pub content_type : String,
    pub body_string : String,
    pub length : usize,
    pub iscgi : bool
}

pub fn parser (s : String) -> Pkg {

    let le = s.len();

    let mut host = String::new();
    let mut user = String::new();
    let mut path = String::new();
    let mut query_string = String::new();
    let mut content_length = String::new();
    let mut content_type = String::new();
    let mut body_string = String::new();
    let mut fore_string = String::new();

    let mut splitreq = s.split("\r\n\r\n");
    let mut rpart = 0;
    for sr in splitreq {
        if rpart == 0 {
            fore_string = sr.to_string();
        }
        else {
            body_string = sr.to_string();
        }
        rpart = rpart + 1;
    }  

    let su = &fore_string.as_bytes();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers[..]);
    let res = req.parse(su).unwrap();

    let mut iscgi = false;
    let parturl: String = req.path.unwrap().chars().take(8).collect();
    if parturl == "/cgi-bin" {
        iscgi = true;
    }

    let method = req.method.unwrap().to_string();
    let mut url = req.path.unwrap().to_string();
    let mut spliturl = url.split("?"); 
    let mut part = 0;
    for su in spliturl {
        if part == 0 {
            path = su.to_string().chars().skip(1).collect();
        }
        else {
            query_string = su.to_string();
        }
        part = part + 1;
    }

    let mut index = 0;
    while index < 16 {
        let i = req.headers[index];
        if i.name.to_string() == "Host" {
            host = String::from_utf8(i.value.to_vec()).unwrap();
        }

        if i.name.to_string() == "User-Agent" {
            user = String::from_utf8(i.value.to_vec()).unwrap();
        }

        if i.name.to_string() == "Content-Length" {
            content_length = String::from_utf8(i.value.to_vec()).unwrap();
        }

        if i.name.to_string() == "Content-Type" {
            content_type = String::from_utf8(i.value.to_vec()).unwrap();
        }

        index = index + 1;
    }
    
    Pkg {
            method: method,
            host: host,
            user: user,
            url: url,
            path: path,
            query_string: query_string,
            content_length: content_length,
            content_type: content_type,
            body_string: body_string,
            length: le,
            iscgi: iscgi

        }
}

#[cfg(test)]
mod parser_tests {
    use crate::parser;

    #[test]
    fn test1() {
        let get1 = "GET /cgi-bin/calculator.py?value1=123&value2=234 HTTP/1.1\r\nHost: localhost:8000\r\nUpgrade-Insecure-Requests: 1\r\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.75 Safari/537.36\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9\r\nSec-Fetch-Site: cross-site";
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/calculator.py";
        let pkgp = parser(get1.to_string());
        assert_eq!(query_string, pkgp.query_string);
        assert_eq!(path, pkgp.path);
    }

    #[test]
    fn post1() {
        let post1 = "POST /cgi-bin/calculator.py?value1=123&value2=234 HTTP/1.1\r\nHost: localhost:8000\r\nContent-Type:application/json\r\nContent-Length:200\r\nAccept:application/json\r\n\r\ntest body string";
        let query_string = "value1=123&value2=234";
        let path = "cgi-bin/calculator.py";
        let content_type = "application/json";
        let body_string = "test body string";
        let pkgp = parser(post1.to_string());
        assert_eq!(query_string, pkgp.query_string);
        assert_eq!(path, pkgp.path);
    }


}