use clap::Parser;
use std::fs;

mod prob01;
mod prob02;

/// "#AOC2023"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Problem number (01, 02, ...)
    #[arg(short, long)]
    prob: u64,
    /// Sub-problem (a or b)
    #[arg(short, long)]
    sub_prob: String,
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let input = fs::read_to_string(&args.input).expect("bad input");
    if args.prob == 1 {
        if args.sub_prob == "a" {
            println!("{}", prob01::a(input));
        } else if args.sub_prob == "b" {
            println!("{}", prob01::b(input));
        }
    } else if args.prob == 2 {
        if args.sub_prob == "a" {
            println!("{}", prob02::a(input));
        } else if args.sub_prob == "b" {
            println!("{}", prob02::b(input));
        }
    }
}
