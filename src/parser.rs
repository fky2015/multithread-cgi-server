
extern crate httparse;
use httparse::Request;

#[derive(Debug)]
pub struct Pkg {
    method : String,
    host : String,
    user : String,
    length : usize,
    iscgi : bool
}

pub fn parser (s : String) -> Pkg {
    let le = s.len();
    let su = s.as_bytes();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers[..]);
    let res = req.parse(su).unwrap();

    let mut iscgi = false;
    let partpath: String = req.path.unwrap().chars().take(8).collect();
    if partpath == "/cgi-bin" {
        iscgi = true;
    }
    
    Pkg {
            method: req.method.unwrap().to_string(),
            host: String::from_utf8(req.headers[0].value.to_vec()).unwrap(),
            user: String::from_utf8(req.headers[4].value.to_vec()).unwrap(),
            length: le,
            iscgi: iscgi
        }

}

