use crate::day_2::{Direction, Move};

pub fn solve(input: Vec<Move>) -> i64 {
    let final_position = input
        .into_iter()
        .fold(Position { horizontal: 0, depth: 0, aim: 0 }, |acc: Position, current| {
            match current.direction {
                Direction::Forward => Position { horizontal: acc.horizontal + current.distance, depth: acc.depth + current.distance * acc.aim, ..acc},
                Direction::Up => Position { aim: acc.aim - current.distance, ..acc},
                Direction::Down => Position { aim: acc.aim + current.distance, ..acc}
            }
        });

    final_position.horizontal * final_position.depth
}

struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64
}