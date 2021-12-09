fn read_data() -> Vec<u16> {

    let contents = std::fs::read_to_string("data")
        .expect("File could no be read");

    let data = contents.lines()
                            .map(|s| u16::from_str_radix(s, 2)
                            .unwrap())
                            .collect::<Vec<_>>();

    return data;
}

fn get_bit(number: &u16, index: &usize) -> bool {
    let mask = 1 << index;
    return number & mask > 0;
}

fn calculate_gamma(data: &Vec<u16>) -> u16 {

    let mut counts: [usize; 12] = [0; 12];

    for number in data {
        for index in 0..12 {
            if get_bit(number, &index) {
                counts[index] += 1;
            }
        }
    }

    let gamma = counts
        .iter()
        .enumerate()
        .fold(0, |bitsum, (index, count)| {
            let bit: u16 = if count > &(&data.len() / 2) { 1 } else { 0 };
            return bitsum | (bit << index);
        });

    return gamma;
}

fn calculate_epsilon_from_gamma(gamma: &u16) -> u16 {
    let epsilon = !gamma & 0x0FFF;
    return epsilon;
}

fn task1() {

    let data = read_data();

    let gamma = calculate_gamma(&data);
    let epsilon = calculate_epsilon_from_gamma(&gamma);

    println!("Result: {}", gamma as u32 * epsilon as u32);
}

fn task2() {

}

fn main() {
    println!("Task 1");
    task1();

    println!("Task 2");
    task2();
}
