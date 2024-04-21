use httparse::Request;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    runtime::Runtime,
};

use crate::primes::is_prime;

pub fn listen() {
    Runtime::new().unwrap().block_on(async {
        async_listen().await;
    })
}

async fn async_listen() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Tokio Async - Server Started");

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_stream(stream).await;
        });
    }
}

async fn handle_stream(mut stream: TcpStream) {
    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);

    let buf_reader = BufReader::new(&mut stream);

    let mut lines_reader = buf_reader.lines();
    let mut lines: Vec<String> = Vec::new();

    while let Some(line) = lines_reader.next_line().await.unwrap() {
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }

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
    let res = handle_request(req).await;

    stream.write_all(res.as_bytes()).await.unwrap();
}

const BAD_REQUEST: &'static str = "HTTP/1.1 401 BAD REQUEST\r\n\r\n";

async fn handle_request<'a>(req: Request<'a, 'a>) -> String {
    let num_header = match req.headers.iter().find(|&&h| h.name == "Number") {
        None => return BAD_REQUEST.to_string(),
        Some(v) => v,
    };

    let mut num_str = String::new();
    let Ok(_) = num_header
        .to_owned()
        .value
        .read_to_string(&mut num_str)
        .await
    else {
        return BAD_REQUEST.to_string();
    };

    let num: u64 = match num_str.parse() {
        Ok(v) => v,
        Err(_) => return BAD_REQUEST.to_string(),
    };

    let res = match tokio::task::spawn_blocking(move || is_prime(num))
        .await
        .unwrap()
    {
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
