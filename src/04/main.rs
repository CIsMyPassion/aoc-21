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
        //println!("{}", s);
        let mut values: [[u8; 5]; 5] = [[0; 5]; 5];
        let rows = s.split("\n").collect::<Vec<_>>();
        //println!("{}", rows.len());

        for (i, r) in rows.iter().enumerate() {
            let row_trim = r.trim().replace("  ", " ");
            let elements = row_trim.split(" ").collect::<Vec<_>>();
            //println!("{}", elements.len());
            //println!("{}", r);

            for (j, e) in elements.iter().enumerate() {
                values[j][i] = e.parse::<u8>().unwrap();
                //println!("{} {} {}", values[j][i], j, i);
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

fn task1() {

    let (numbers, boards) = read_data();

    println!("{} {}", numbers.len(), boards.len());

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
