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

}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: [[u8; 5]; 5] = [[0; 5]; 5];
        let rows = s.split("\n").collect::<Vec<_>>();

        for (i, r) in rows.iter().enumerate() {
            let elements = r.split(" ").collect::<Vec<_>>();

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

    let board_string = &contents[end_of_line..];

    (numbers, boards)
}

fn task1() {

    let (numbers, boards) = read_data();

    println!("{} {}", numbers.len(), boards.len());

    let vals = [[5; 5]; 5];
    let board = Board::new(vals);

    println!("{} {}", board.picked[0][0], board.values[0][0]);

    println!("Result: {}", 0);
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
