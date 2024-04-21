use clap::{Parser, ValueEnum};

pub mod async_based;
pub mod primes;
pub mod thread_based;

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
    /// OS Thread Pool
    ThreadPool,
}

fn main() {
    let args = Args::parse();

    match args.mode {
        Mode::Async => async_based::listen(),
        Mode::ThreadPool => thread_based::listen(),
    }
}
