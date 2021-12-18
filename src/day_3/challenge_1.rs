pub fn solve(input: Vec<Vec<u32>>) -> u32 {
    let mut gamma_rate: u32 = 0;
    let data_size = input.len();
    let word_size = input.first().unwrap().len();
    for i in 0..word_size { 
        let ones_count: u32 = input
            .iter()
            .map(|row| row.iter().nth(i).unwrap())
            .sum();
        if ones_count > (data_size / 2) as u32 {
            gamma_rate <<= 1;
            gamma_rate |= 0b1;
        } else {
            gamma_rate <<= 1;
        }
    }
    
    return gamma_rate * ((!gamma_rate) & (1 << word_size) - 1);
}