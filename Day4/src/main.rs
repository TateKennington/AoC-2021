use std::{collections::HashMap, fs::read_to_string};

struct Board {
    pub positions: HashMap<u32, (u32, u32)>,
    pub rows: [u32; 5],
    pub columns: [u32; 5],
    sum: u32,
    won: bool,
}

fn main() {
    let input = read_to_string("in").unwrap();
    let mut boards = input.split("\r\n\r\n");
    let order = boards.next().unwrap();
    let mut boards: Vec<_> = boards
        .map(|board| {
            let mut parsed_board = Board {
                positions: HashMap::new(),
                rows: [5, 5, 5, 5, 5],
                columns: [5, 5, 5, 5, 5],
                sum: 0,
                won: false,
            };
            board
                .lines()
                .map(|line| line.trim())
                .enumerate()
                .for_each(|(row, line)| {
                    line.split_whitespace()
                        .enumerate()
                        .for_each(|(column, value)| {
                            let value: u32 = value.parse().unwrap();
                            parsed_board.sum += value;
                            parsed_board
                                .positions
                                .insert(value, (row as u32, column as u32));
                        })
                });
            parsed_board
        })
        .collect();
    let mut first = None;
    let mut last = 0;
    order.split(',').for_each(|number| {
        let number: u32 = number.trim().parse().unwrap();
        boards
            .iter_mut()
            .filter(|board| !board.won)
            .for_each(|board| {
                if let Some((row, col)) = board.positions.get(&number) {
                    board.rows[*row as usize] -= 1;
                    board.columns[*col as usize] -= 1;
                    board.sum -= number;

                    if board.rows[*row as usize] == 0 || board.columns[*col as usize] == 0 {
                        board.won = true;
                        if first.is_none() {
                            first = Some(board.sum * number);
                        }
                        last = board.sum * number;
                    }
                }
            })
    });

    println!("{}", first.unwrap());
    println!("{}", last);
}
