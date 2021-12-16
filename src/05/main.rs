struct Point {
    x: u16,
    y: u16,
}

struct Line {
    start: Point,
    end: Point,
}

fn task1() {

    let line = Line{ start: Point{ x: 0, y: 0 }, end: Point{ x: 0, y: 0 } };

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
