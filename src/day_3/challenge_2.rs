pub fn solve(input: Vec<Vec<u32>>) -> u32 {
    let word_size = input.first().unwrap().len();
    //let oxygen = get_oxygen_generation_rating(input.clone(), data_size, word_size);
    let mut oxygen_generator_rating: Vec<Vec<u32>> = vec![];
    let mut co2_scrubber_rating: Vec<Vec<u32>> = vec![];

    for i in 0..word_size {
        if i == 0 {
            let most_common_bit = find_most_common_bit(&input, i);
            if most_common_bit == 1 {
                oxygen_generator_rating = filter_data(&input, i, 1);
                co2_scrubber_rating = filter_data(&input, i, 0);
            } else {
                oxygen_generator_rating = filter_data(&input, i, 0);
                co2_scrubber_rating = filter_data(&input, i, 1);
            }
        } else {
            if oxygen_generator_rating.len() > 1 {
                let oxygen_most_common_bit = find_most_common_bit(&oxygen_generator_rating, i);
                if oxygen_most_common_bit == 1 {
                    oxygen_generator_rating = filter_data(&oxygen_generator_rating, i, 1);
                } else {
                    oxygen_generator_rating = filter_data(&oxygen_generator_rating, i, 0);
                }
            }
            
            if co2_scrubber_rating.len() > 1 {
                let co2_most_common_bit = find_most_common_bit(&co2_scrubber_rating, i);
                if co2_most_common_bit == 1 {
                    co2_scrubber_rating = filter_data(&co2_scrubber_rating, i, 0);
                } else {
                    co2_scrubber_rating = filter_data(&co2_scrubber_rating, i, 1);
                }
            }
        }

        if oxygen_generator_rating.len() == 1 && co2_scrubber_rating.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = merge_numbers(oxygen_generator_rating.first().unwrap());
    let co2_scrubber_rating = merge_numbers(co2_scrubber_rating.first().unwrap());
    
    oxygen_generator_rating * co2_scrubber_rating
}

fn merge_numbers(data: &Vec<u32>) -> u32 {
    data
        .iter()
        .fold(0, |acc, value| (acc << 1) | value)
}

fn find_most_common_bit(data: &Vec<Vec<u32>>, bit_to_find: usize) -> u32 {
    let data_size = data.len();
    let ones_count: u32 = data
        .iter()
        .map(|row| row.iter().nth(bit_to_find).unwrap())
        .sum();
    if ones_count >= (data_size as f32 / 2 as f32).ceil() as u32 {
        1
    } else {
        0
    }
}

fn filter_data(data: &Vec<Vec<u32>>, bit_to_filter: usize, bit_value: u32) -> Vec<Vec<u32>> {
    data
        .iter()
        .filter_map(|row| {
            if *row.iter().nth(bit_to_filter).unwrap() == bit_value {
                Some(row.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}