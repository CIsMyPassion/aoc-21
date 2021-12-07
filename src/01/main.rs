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

fn calculate_derivatives(data: Vec<i32>) -> Vec<Derivative> {
    let mut derivatives = Vec::new();

    for i in 1..data.len() {
        if data[i] < data[i - 1] {
            derivatives.push(Derivative::Decreasing);
        } else {
            derivatives.push(Derivative::Increasing);
        }
    }

    return derivatives;
}

fn main() {

    // read data form file
    let data = read_data();
    // calculate the derivatives
    let derivatives = calculate_derivatives(data);

    // count increasing and decreasing derivaties
    let increase_count = derivatives.iter().filter(|&d| *d == Derivative::Increasing).count();
    let decrease_count = derivatives.iter().filter(|&d| *d == Derivative::Decreasing).count();

    // output count of increasing and decreasing derivatives
    println!("Increasing: {}\nDecreasing: {}", increase_count, decrease_count);
}
