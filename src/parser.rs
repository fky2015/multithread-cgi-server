
extern crate httparse;
use httparse::Request;

struct Pkg {
    method : String,
    host : String,
    user : String,
    length : usize,
    iscgi : bool
}

fn parser (s : String) -> Pkg {
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
    
    let pkg1 = Pkg {
            method: req.method.unwrap().to_string(),
            host: String::from_utf8(req.headers[0].value.to_vec()).unwrap(),
            user: String::from_utf8(req.headers[4].value.to_vec()).unwrap(),
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
