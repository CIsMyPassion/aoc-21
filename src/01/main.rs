fn read_data() -> Vec<i32> {

    let contents = std::fs::read_to_string("data")
        .expect("File could no be read");

    println!("Data is:\n{}", contents);

    let mut data = Vec::new();

    data.push(199);
    data.push(200);
    data.push(203);
    data.push(201);

    return data;
}

#[derive(Debug)]
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
    let data = read_data();
    println!("Length: {}", data.len());
    let derivatives = calculate_derivatives(data);
    println!("Length: {}", derivatives.len());

    for d in derivatives {
        println!("{:?}", d);
    }
}
