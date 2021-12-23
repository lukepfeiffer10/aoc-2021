use std::collections::HashMap;
use std::fmt::{Debug};

pub(crate) mod challenge_1;
pub(crate) mod challenge_2;

pub fn parse<'a, I>(input: I) -> BingoGame
    where I: Iterator<Item = &'a str> {
    let mut first = true;
    let mut called_numbers: Vec<i32> = vec![];
    let mut board_number = 0;
    let mut row_number = 0;
    let mut board_positions = HashMap::new();
    for value in input {
        if first {
            called_numbers = value
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect();
            first = false;
            continue;
        }
        if value.is_empty() {
            board_number += 1;
            row_number = 0;
            continue;
        }
        let enumerated_values = value
            .split(' ')
            .filter(|c| !c.is_empty())
            .enumerate();
        
        for (i, c) in enumerated_values {
            let bingo_number = c.parse::<i32>().unwrap();
            let board_position = BoardPosition {
                board: board_number,
                row: row_number,
                column: i
            };
            match board_positions.get_mut(&bingo_number) {
                None => {
                    board_positions.insert(bingo_number, vec![board_position]);
                },
                Some(position_list) => position_list.push(board_position) 
            };
        }
        row_number += 1;
    };

    BingoGame { 
        called_numbers,
        number_of_boards: board_number,
        board_positions
    }
}

#[derive(Debug)]
pub struct BoardPosition {
    pub board: usize,
    pub row: usize,
    pub column: usize
}

pub struct BingoGame {
    pub called_numbers: Vec<i32>,
    pub number_of_boards: usize,
    pub board_positions: HashMap<i32, Vec<BoardPosition>>
}

// Holds the count of the number of marked numbers in each row and column
#[derive(Clone)]
struct BoardCounter {
    rows: Vec<i32>,
    columns: Vec<i32>
}