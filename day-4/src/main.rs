use std::fs;

struct Board { 
    data: Vec<Vec<u32>>,
    won: bool
}

const BOARD_WIDTH: usize = 5;

fn parse_boards(input: String) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let raw_boards = input.split("\n\n").skip(1);

    for i in raw_boards {
        let mut data = Vec::new();
        let mut checked = Vec::new();

        for line in i.lines() {

            let nums: Vec<u32> = line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();

            checked.push(vec![false; nums.len()]);
            data.push(nums);
        }

        boards.push(Board { data, won: false });
    };

    boards
}

fn solve_game(winning_numbers: Vec<u32>, boards: &mut Vec<Board>) -> Vec<(usize, u32)> {
    let mut winning_boards: Vec<(usize, u32)> = Vec::new();

    for i in (BOARD_WIDTH - 1)..winning_numbers.len() {
        let current = &winning_numbers.get(0..i + 1).unwrap();

        for board in boards.iter_mut() {
            if !board.won {
                // we iterate every row
                for row in board.data.iter() {
                    if !board.won && row.iter().all(|&item| current.contains(&item)) {
                        let sum_unchecked = board.data.iter().flatten().filter(|x| !current.contains(&x)).fold(0, |acc, x| acc + x);
                        winning_boards.push((current.len(), sum_unchecked * current.last().unwrap()));
                        board.won = true;
                    }
                }

                // we interate every column
                for column_index in 0..BOARD_WIDTH {
                    if !board.won && (board.data.iter().map(|line| line.get(column_index).unwrap()).all(|item| current.contains(item))) {
                        let sum_unchecked = board.data.iter().flatten().filter(|x| !current.contains(&x)).fold(0, |acc, x| acc + x);
                        winning_boards.push((current.len(), sum_unchecked * current.last().unwrap()));
                        board.won = true;
                    }
                }
            }
        }
    }

    return winning_boards;
}

fn main() {
    let csv_file_path = "./data.txt";
    let data = fs::read_to_string(csv_file_path).expect("Unable to read file");

    let mut lines = data.lines();

    let winning_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut boards = parse_boards(data);

    let result = solve_game(winning_numbers, &mut boards);
    let (score, last_sum) = result.last().unwrap();

    dbg!(score, last_sum);
}
