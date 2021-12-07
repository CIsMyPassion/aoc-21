fn read_data() -> Vec<i32> {

    let contents = std::fs::read_to_string("data")
        .expect("File could no be read");


    let lines = contents.lines().collect::<Vec<_>>();
    let data = lines.into_iter().map(String::from)
                                .map(|s| s.parse().unwrap())
                                .collect();

    return data;
}

#[derive(PartialEq)]
enum Derivative {
    Increasing,
    Decreasing
}

fn calculate_derivatives(data: &Vec<i32>) -> Vec<Derivative> {
    let mut derivatives = Vec::new();

    for i in 1..data.len() {
        if data[i] <= data[i - 1] {
            derivatives.push(Derivative::Decreasing);
        } else {
            derivatives.push(Derivative::Increasing);
        }
    }

    return derivatives;
}

fn task1() {

    // read data form file
    let data = read_data();
    // calculate the derivatives
    let derivatives = calculate_derivatives(&data);

    // count increasing and decreasing derivaties
    let increase_count = derivatives.iter().filter(|&d| *d == Derivative::Increasing).count();
    let decrease_count = derivatives.iter().filter(|&d| *d == Derivative::Decreasing).count();

    // output count of increasing and decreasing derivatives
    println!("Increasing: {}\nDecreasing: {}", increase_count, decrease_count);
}

fn calculate_sliding_window(data: &Vec<i32>) -> Vec<i32> {

    let mut sliding_window = Vec::new();

    for i in data.windows(3) {
        let sum = i.iter().sum::<i32>();
        sliding_window.push(sum);
    }

    return sliding_window;
}

fn task2() {

    // read data form file
    let data = read_data();
    // calculate sliding window
    let sliding_window = calculate_sliding_window(&data);
    // calculate derivatives
    let derivatives = calculate_derivatives(&sliding_window);

    println!("Len {} Len {} Len {}", data.len(), sliding_window.len(), derivatives.len());

    // count increasing and decreasing derivaties
    let increase_count = derivatives.iter().filter(|&d| *d == Derivative::Increasing).count();
    let decrease_count = derivatives.iter().filter(|&d| *d == Derivative::Decreasing).count();

    // output count of increasing and decreasing derivatives
    println!("Increasing: {}\nDecreasing: {}", increase_count, decrease_count);
}

fn main() {
    println!("Task 1:");
    task1();

    println!("Task 2:");
    task2();
}
