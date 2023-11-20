use clap::Parser;

mod dlls;
mod loader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    exec: String
}

fn main() {
    let args = Args::parse();

    println!("Hello, world!");
}
