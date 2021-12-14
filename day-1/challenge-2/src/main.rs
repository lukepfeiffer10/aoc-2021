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
        .split("\n")
        .filter(|value| !value.is_empty())
        .map(|value| value.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut increases = 0;
    let mut first_window_start = 0;
    let mut first_window_end = 2;
    let mut second_window_start = 1;
    let mut second_window_end = 3;
    while second_window_end < input.len() {
        let first_window_sum = &input[first_window_start..=first_window_end].iter().sum::<i64>();
        let second_window_sum = &input[second_window_start..=second_window_end].iter().sum::<i64>();
        
        if second_window_sum > first_window_sum {
            increases += 1;
        }
        
        first_window_start += 1;
        first_window_end += 1;
        second_window_start += 1;
        second_window_end += 1;
     }

    println!("The number of increases is: {}", increases)
}
