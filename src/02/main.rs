enum Direction {
    Forward(u32),
    Backward(u32),
    Up(u32),
    Down(u32)
}

impl std::str::FromStr for Direction {

    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {

        let space_index = input.find(' ').unwrap();
        let first_part = &input[..space_index];
        let second_part = &input[(space_index + 1)..];
        let value: u32 = second_part.parse().unwrap();

        match first_part {
            "forward" => Ok(Direction::Forward(value)),
            "backward" => Ok(Direction::Backward(value)),
            "up" => Ok(Direction::Up(value)),
            "down" => Ok(Direction::Down(value)),
            _ => Err(()),
        }
    }
}

fn read_data() -> Vec<Direction> {

    let contents = std::fs::read_to_string("data")
        .expect("File could no be read");


    let lines = contents.lines().collect::<Vec<_>>();
    let data = lines.into_iter().map(String::from)
                                .map(|s| s.parse::<Direction>().unwrap())
                                .collect();

    return data;
}

fn calculate_position(data: &Vec<Direction>) -> (i32, i32) {

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for direction in data {
        match direction {
            Direction::Forward(value) => x += *value as i32,
            Direction::Backward(value) => x -= *value as i32,
            Direction::Up(value) => y -= *value as i32,
            Direction::Down(value) => y += *value as i32,
        }
    }

    return (x, y);
}

fn calculate_position_with_aim(data: &Vec<Direction>) -> (i32, i32) {

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;

    for direction in data {
        match direction {
            Direction::Forward(value) => {
                x += *value as i32;
                y += aim * *value as i32
            },
            Direction::Backward(value) => x -= *value as i32,
            Direction::Up(value) => aim -= *value as i32,
            Direction::Down(value) => aim += *value as i32,
        }
    }

    return (x, y);
}

fn task1() {

    let data = read_data();
    let (x, y) = calculate_position(&data);

    println!("Result: {}", x * y);
}

fn task2() {

    let data = read_data();
    let (x, y) = calculate_position_with_aim(&data);

    println!("Result: {}", x * y);
}

fn main() {
    println!("Task 1:");
    task1();

    println!("Task 2:");
    task2();
}
