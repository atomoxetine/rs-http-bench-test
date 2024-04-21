use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use httparse::Request;

use crate::primes::is_prime;

pub const POOL_SIZE: usize = 200;

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(POOL_SIZE)
        .build()
        .unwrap();

    println!("OS Thread Pool - Server Started");
    println!("{} Threads on pool", POOL_SIZE);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.install(|| handle_stream(stream))
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);

    let buf_reader = BufReader::new(&mut stream);

    let mut lines: Vec<String> = buf_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_str: String = lines
        .iter_mut()
        .map(|s| {
            s.push_str("\r\n");
            s
        })
        .flat_map(|s| s.chars())
        .collect();

    let bytebuf = request_str.into_bytes();

    let _ = req.parse(&bytebuf);
    let res = handle_request(req);

    stream.write_all(res.as_bytes()).unwrap();
}

const BAD_REQUEST: &'static str = "HTTP/1.1 401 BAD REQUEST\r\n\r\n";

fn handle_request(req: Request) -> String {
    let num_header =
        match req.headers.iter().find(|&&h| h.name == "Number") {
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
