pub(crate) mod challenge_1;
pub(crate) mod challenge_2;

pub fn parse<'a, I>(input: I) -> Vec<Vec<u32>>
    where I: Iterator<Item = &'a str> {
    let mut data: Vec<Vec<u32>> = vec![];    
    for value in input {
        let value = value
            .chars()
            .map(|c| c.to_digit(2).unwrap())
            .collect::<Vec<_>>();
        data.push(value);
    };

    data
}