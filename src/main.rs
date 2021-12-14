mod day_1;
mod day_2;

use std::env::args;
use std::fs;

fn main() {
    let arguments = args();
    if arguments.len() == 1 {
        panic!("Must supply a file as input!")
    }

    let filename = arguments
        .skip(1)
        .next()
        .unwrap();
    let input = fs::read_to_string(filename)
        .unwrap();
    let input = day_2::parse(input.lines());
        
    
    println!("Answer: {}", day_2::challenge_2::solve(input))
}