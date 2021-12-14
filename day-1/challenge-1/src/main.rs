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
    let input = input
        .lines()
        .map(|value| value.parse::<i64>().unwrap());
    let mut increases = 0;
    let mut current = 0;
    let mut first = true;
    for value in input {
        if !first && value > current {
            increases += 1;
        }
        first = false;
        current = value;
    }
    
    println!("The number of increases is: {}", increases)
}
