pub(crate) mod challenge_1;
pub(crate) mod challenge_2;

pub fn parse<'a, I>(input: I) -> Vec<Move>
    where I: Iterator<Item = &'a str> {
    input
        .map(|value| { 
            let split_value = value.split(' ').collect::<Vec<_>>();
            let direction_text = split_value[0];
            let distance = split_value[1].parse::<i64>().unwrap();
            match direction_text {
                "forward" => Move{ direction: Direction::Forward, distance },
                "up" => Move{ direction: Direction::Up, distance },
                "down" => Move{ direction: Direction::Down, distance },
                _ => panic!("Bad input: {}", direction_text)
            }
        })
        .collect()
}

pub enum Direction {
    Forward,
    Up,
    Down
}

pub struct Position {
    pub horizontal: i64,
    pub depth: i64
}

pub struct Move {
    pub direction: Direction,
    pub distance: i64
}