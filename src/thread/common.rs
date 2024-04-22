use std::io::Read;

use httparse::Request;

use crate::primes::is_prime;

const BAD_REQUEST: &str = "HTTP/1.1 401 BAD REQUEST\r\n\r\n";

pub fn handle_request(req: Request) -> String {
    let num_header = match req.headers.iter().find(|&&h| h.name == "Number") {
        None => return BAD_REQUEST.to_string(),
        Some(v) => v,
    };

    let mut num_str = String::new();
    let Ok(_) = num_header.to_owned().value.read_to_string(&mut num_str) else {
        return BAD_REQUEST.to_string();
    };

    let num: u64 = match num_str.parse() {
        Ok(v) => v,
        Err(_) => return BAD_REQUEST.to_string(),
    };

    let res = match is_prime(num) {
        Some(t) => match t {
            true => "yes",
            false => "no",
        },
        None => "maybe",
    };

    let body = format!("is {} prime? {}", num, res);
    let len = body.len();

    format!("HTTP/1.1 200 OK\r\nContent-Length: {len}\r\n\r\n{body}")
}
