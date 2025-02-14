use clap::Parser;
use structs::Cli;

pub mod structs;

fn main() {
    let args = Cli::parse();
    println!("{:?}, {:?}", args.path(), args.pattern());
    let current = std::fs::read_to_string(args.path()).expect("could not read file");
    println!("{:?}", current);

    for line in current.lines() {
        if line.contains(args.pattern()) {
            println!("{:?}", line)
        }
    } 
}
