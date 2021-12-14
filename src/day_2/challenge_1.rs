use crate::day_2::{Direction, Move, Position};

pub fn solve(input: Vec<Move>) -> i64 {
    let final_position = input
        .into_iter()
        .fold(Position { horizontal: 0, depth: 0 }, |acc: Position, current| {            
            match current.direction {
                Direction::Forward => Position { horizontal: acc.horizontal + current.distance, ..acc},
                Direction::Up => Position { depth: acc.depth - current.distance, ..acc},
                Direction::Down => Position { depth: acc.depth + current.distance, ..acc}
            }
        });
    
    final_position.horizontal * final_position.depth
}