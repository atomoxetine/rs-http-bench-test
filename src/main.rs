use clap::{Parser, ValueEnum};

pub mod primes;
pub mod pure_async;
pub mod thread;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// What mode to run
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Async with Tokio
    Async,
    /// 1 OS Thread per Connection
    ThreadPerConn,
    /// 1 OS Thread per Request
    ThreadPerReq,
}

fn main() {
    let args = Args::parse();

    match args.mode {
        Mode::Async => pure_async::listen(),
        Mode::ThreadPerConn => thread::thread_per_conn::listen(),
        Mode::ThreadPerReq => thread::thread_per_req::listen(),
    }
}
