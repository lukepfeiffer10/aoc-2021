pub(crate) mod challenge_1;
pub(crate) mod challenge_2;

pub fn parse<'a, I>(input: I) -> Vec<i64>
    where I: Iterator<Item = &'a str> {
    input
        .map(|value| value.parse::<i64>().unwrap())
        .collect()
}