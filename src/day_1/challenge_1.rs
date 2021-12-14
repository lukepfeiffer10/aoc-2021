pub fn solve(input: Vec<i64>) -> i64 {
    let mut increases = 0;
    for i in 0..input.len() - 1 {
        if input[i+1] > input[i] {
            increases += 1;
        }
    }
    
    increases
}
