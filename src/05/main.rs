use std::str::FromStr;
use std::fs;

#[derive(Debug, Clone, Copy)]
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
    values: Vec<u8>,
    width: u16,
    height: u16,
}

impl Field {

    pub fn new(width: u16, height: u16) -> Self {

        let length = width as usize * height as usize;

        Field { values: vec![0; length], width: width, height: height }
    }

    pub fn get_value(&self, point: &Point) -> u8 {
        let position = point.y as usize * self.width as usize + point.x as usize;
        self.values[position]
    }

    fn apply_points(&mut self, points: &Vec<Point>) {
        for point in points {
            let position = point.y as usize * self.width as usize + point.x as usize;
            self.values[position] += 1;
        }
    }
}

fn read_data() -> Vec<Line> {

    let contents = fs::read_to_string("data")
        .expect("File could not be read");

    let lines_strings = contents.split("\n").filter(|s| s.len() > 0).collect::<Vec<_>>();
    let mut lines: Vec<Line> = Vec::new();

    for line_string in lines_strings {
        lines.push(line_string.parse::<Line>().unwrap());
    }

    lines
}

fn task1() {

    let lines = read_data();
    let mut field = Field::new(1000, 1000);

    for line in &lines {

        if line.start.x == line.end.x || line.start.y == line.end.y {


            let mut points = Vec::new();

            let x_min = std::cmp::min(line.start.x, line.end.x);
            let x_max = std::cmp::max(line.start.x, line.end.x) + 1;
            let y_min = std::cmp::min(line.start.y, line.end.y);
            let y_max = std::cmp::max(line.start.y, line.end.y) + 1;

            for x in x_min..x_max {
                for y in y_min..y_max {
                    points.push(Point::new(x, y));
                }
            }

            field.apply_points(&points);
        }
    }

    let mut counter = 0;

    for x in 0..field.width {
        for y in 0..field.height {
            if field.get_value(&Point::new(x, y)) >= 2 {
                counter += 1;
            }
        }
    }

    println!("Result: {}", counter);
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
