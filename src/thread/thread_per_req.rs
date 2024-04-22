use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use rayon::ThreadPool;

use super::common::handle_request;

pub const POOL_SIZE: usize = 400;

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    let pool_lock = Arc::new(Mutex::new(
        rayon::ThreadPoolBuilder::new()
            .num_threads(POOL_SIZE)
            .build()
            .unwrap(),
    ));

    println!("OS Thread Pool - Server Started");
    println!("1 Thread per request");
    println!("{} Threads on pool", POOL_SIZE);

    loop {
        let (stream, _) = listener.accept().unwrap();

        let pool_lock = pool_lock.clone();
        std::thread::spawn(move || handle_stream(stream, pool_lock));
    }
}

fn handle_stream(mut stream: TcpStream, pool_lock: Arc<Mutex<ThreadPool>>) {
    let mut headers = [httparse::EMPTY_HEADER; 8];
    let mut req = httparse::Request::new(&mut headers);
    let mut bytebuf = [0u8; 1024];

    while stream.read(&mut bytebuf).unwrap_or(0) > 0 {
        let Ok(_) = req.parse(&bytebuf) else {
            break;
        };

        let res = {
            let pool = pool_lock.lock().unwrap();
            pool.install(move || handle_request(req))
        };

        stream.write_all(res.as_bytes()).unwrap();
        stream.flush().unwrap();

        headers = [httparse::EMPTY_HEADER; 8];
        req = httparse::Request::new(&mut headers);
    }
}
