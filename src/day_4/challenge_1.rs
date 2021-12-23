use crate::day_4::{BingoGame, BoardCounter};

pub fn solve(mut game: BingoGame) -> i32 {
    let mut boards = vec![BoardCounter {
        rows: vec![0; 5],
        columns: vec![0; 5]
    }; game.number_of_boards];
    let mut final_called_number = 0;
    let mut winning_board_number = 0;
    'outer: for called_number in game.called_numbers {
        match game.board_positions.get(&called_number) {
            None => {}
            Some(positions) => {
                for position in positions {
                    let board = &mut boards[(position.board - 1)];
                    board.rows[position.row] += 1; 
                    board.columns[position.column] += 1;
                    if board.rows[position.row] == 5 || 
                        board.columns[position.column] == 5 {
                        
                        final_called_number = called_number;
                        winning_board_number = position.board;
                        break 'outer;
                    }
                }
            }
        }
        game.board_positions.remove(&called_number);
    }

    game.board_positions.remove(&final_called_number);
    let winning_board_unmarked_sum: i32 = game.board_positions
        .iter()
        .flat_map(|item| item.1.iter().map(move |position| (item.0, position)))
        .filter_map(|position_value| if position_value.1.board == winning_board_number {
            Some(position_value.0)
        } else {
            None
        })
        .sum();        
    
    final_called_number * winning_board_unmarked_sum
}