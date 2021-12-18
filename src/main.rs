//mod day_1;
//mod day_2;

mod day_3;

use std::env::args;
use std::fs;
use crate::day_3::challenge_2::solve;
use crate::day_3::parse;

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
    let input = parse(input.lines());
        
    
    println!("Answer: {}", solve(input))
}