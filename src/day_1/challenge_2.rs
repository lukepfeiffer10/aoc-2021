pub fn solve(input: Vec<i64>) -> i64 {
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

    increases
}
