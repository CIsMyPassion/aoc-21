use std::str::FromStr;
use std::num::ParseIntError;
use std::fs;

struct Board {
    values: [[u8; 5]; 5],
    picked: [[bool; 5]; 5],
}

impl Board {

    pub fn new(values: [[u8; 5]; 5]) -> Self {
        Self { values: values, picked: [[false; 5]; 5] }
    }

    pub fn choose_number(&mut self, number: &u8) -> bool {
        for (i, row) in self.values.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                if *element == *number {
                    self.picked[i][j] = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn is_solved(&self) -> bool {

        for i in 0..5 {
            if self.check_row(i) {
                return true;
            }
        }

        for i in 0..5 {
            if self.check_column(i) {
                return true;
            }
        }

        false
    }

    fn check_row(&self, index: usize) -> bool {
        for j in 0..5 {
            if !self.picked[index][j] {
                return false;
            }
        }

        true
    }

    fn check_column(&self, index: usize) -> bool {
        for j in 0..5 {
            if !self.picked[j][index] {
                return false;
            }
        }

        true
    }

    pub fn calculate_score(&self, last_number: &u8) -> u16 {
        let mut total_unpicked: u16 = 0;

        for i in 0..5 {
            for j in 0..5 {
                if !self.picked[i][j] {
                    total_unpicked += self.values[i][j] as u16;
                }
            }
        }

        return total_unpicked * *last_number as u16;
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut values: [[u8; 5]; 5] = [[0; 5]; 5];
        let rows = s.split("\n").collect::<Vec<_>>();

        for (i, r) in rows.iter().enumerate() {

            let row_trim = r.trim().replace("  ", " ");
            let elements = row_trim.split(" ").collect::<Vec<_>>();

            for (j, e) in elements.iter().enumerate() {
                values[j][i] = e.parse::<u8>().unwrap();
            }
        }

        Ok(Board::new(values))
    }
}

fn read_data() -> (Vec<u8>, Vec<Board>) {

    let contents = fs::read_to_string("data")
        .expect("File could not be read");

    let end_of_line = contents.find("\n").unwrap();
    let numbers: Vec<u8> = contents[..end_of_line]
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    let mut board_string = &contents[end_of_line..];
    let mut segment_end_index = board_string.find("\n\n");

    while segment_end_index.is_some() {

        let old_index = 2;
        segment_end_index = board_string[old_index..].find("\n\n");

        if let Some(end_index) = segment_end_index {
            let single_board = &board_string[old_index..end_index + 1];

            boards.push(single_board.parse::<Board>().unwrap());
            board_string = &board_string[end_index+2..];

        } else {
            boards.push(board_string.trim().parse::<Board>().unwrap());

        }
    }

    (numbers, boards)
}

fn find_first_winnig_board(numbers: Vec<u8>, mut boards: Vec<Board>) -> u16 {
    for number in &numbers {
        for board in &mut boards {
            if board.choose_number(number) {
                if board.is_solved() {
                    return board.calculate_score(number);
                }
            }
        }
    }

    return 0;
}

fn task1() {

    let (numbers, boards) = read_data();
    let score = find_first_winnig_board(numbers, boards);

    /*
    for number in &numbers {
        for board in &mut boards {
            if board.choose_number(number) {
                if board.is_solved() {
                    let score = board.calculate_score(number);

                    if score > best_score {
                        best_score = score;
                    }
                }
            }
        }
    }
    */

    println!("Result: {}", score);
}

fn task2() {
    println!("Result: {}", 0);
}

fn main() {
    println!("Task 1:");
    task1();

    println!("Task 2:");
    task2();
}
