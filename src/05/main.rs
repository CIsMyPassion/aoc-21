use std::str::FromStr;
use std::fs;

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {

    pub fn new(x: u16, y: u16) -> Self {
        Self { x: x, y: y }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split(",").collect::<Vec<_>>();
        if nums.len() == 2 {
            let x = nums[0].parse::<u16>().unwrap();
            let y = nums[1].parse::<u16>().unwrap();

            Ok(Point::new(x, y))
        } else {
            Err(())
        }
    }

}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {

    pub fn new(start: Point, end: Point) -> Self {
        Self { start: start, end: end }
    }

}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.split(" -> ").collect::<Vec<_>>();
        if points.len() == 2 {
            let start = points[0].parse::<Point>().unwrap();
            let end = points[1].parse::<Point>().unwrap();
            Ok(Line::new(start, end))
        } else {
            Err(())
        }
    }
}

struct Field {
    lines: Vec<Line>,
    values: Vec<u8>,
}

impl Field {

    pub fn new(lines: Vec<Line>) -> Self {
        Field { lines: lines, values: Vec::new() }
    }
}

fn read_data() -> Field {

    let contents = fs::read_to_string("data")
        .expect("File could not be read");

    let lines_strings = contents.split("\n").filter(|s| s.len() > 0).collect::<Vec<_>>();
    let mut lines: Vec<Line> = Vec::new();

    for line_string in lines_strings {
        lines.push(line_string.parse::<Line>().unwrap());
    }

    Field::new(lines)
}

fn task1() {

    let filed = read_data();

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
