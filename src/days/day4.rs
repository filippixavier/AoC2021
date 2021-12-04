use std::error::Error;
use std::fs;
use std::path::Path;

type Board = Vec<Vec<Option<usize>>>;

fn get_boards() -> (Vec<Board>, Vec<usize>) {
    let input =
        fs::read_to_string(Path::new("./data/day4.input")).expect("Error when reading file");
    let mut lines = input.lines();
    let draw = lines.next().unwrap();
    let mut boards: Vec<Board> = vec![];
    let mut board: Board = vec![];
    for line in lines.skip(1) {
        if line.is_empty() {
            boards.push(board);
            board = vec![];
            continue;
        }
        let row = line
            .split_whitespace()
            .map(|num| Some(num.parse::<usize>().unwrap_or(0)))
            .collect();
        board.push(row);
    }

    boards.push(board);

    (
        boards,
        draw.split(',')
            .map(|num| num.parse().unwrap_or(0))
            .collect(),
    )
}

fn check_board(board: Board) -> Option<usize> {
    let len = board.len();

    let mut row_none = true;
    let mut col_none = true;

    for i in 0..len {
        row_none = true;
        col_none = true;
        for j in 0..len {
            if board[i][j].is_some() {
                row_none = false;
            }
            if board[j][i].is_some() {
                col_none = false;
            }
        }

        if row_none || col_none {
            break;
        }
    }

    if row_none || col_none {
        let sum = board.iter().fold(0, |acc, x| {
            acc + x.iter().fold(0, |sub, y| sub + y.unwrap_or(0))
        });
        return Some(sum);
    }
    None
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut boards, draw) = get_boards();

    'named: for num in draw {
        for board in boards.iter_mut() {
            for line in board.iter_mut() {
                let mut index = None;
                for (i, elem) in line.iter().enumerate() {
                    if let Some(val) = elem {
                        if *val == num {
                            index = Some(i);
                            break;
                        }
                    }
                }
                if let Some(i) = index {
                    line[i] = None;
                }
            }
            if let Some(unmarked_sum) = check_board(board.to_vec()) {
                println!("Final score is {}", unmarked_sum * num);
                break 'named;
            }
        }
    }

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (boards, draw) = get_boards();

    let mut boards_with_status: Vec<(bool, Board)> =
        boards.into_iter().map(|board| (false, board)).collect();

    let mut last_round = false;

    'named: for num in draw {
        boards_with_status = boards_with_status
            .into_iter()
            .filter(|(status, _)| !status)
            .collect();
        if boards_with_status.len() == 1 {
            last_round = true;
        }

        for (status, board) in boards_with_status.iter_mut() {
            for line in board.iter_mut() {
                let mut index = None;
                for (i, elem) in line.iter().enumerate() {
                    if let Some(val) = elem {
                        if *val == num {
                            index = Some(i);
                            break;
                        }
                    }
                }
                if let Some(i) = index {
                    line[i] = None;
                }
            }
            if let Some(unmarked_sum) = check_board(board.to_vec()) {
                *status = true;
                if last_round {
                    println!("Last man standing: {}", unmarked_sum * num);
                    break 'named;
                }
            }
        }
    }
    Ok(())
}
