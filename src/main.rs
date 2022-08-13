use std::io;

fn main() {
    let mut input_str: String = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    let input: usize = input_str.trim().parse().unwrap();

    let mut array: Vec<usize> = vec![0; input];

    for i in 0..array.len() {
        let mut number: String = String::new();
        io::stdin().read_line(&mut number).unwrap();
        array[i] = number.trim().parse::<usize>().unwrap();
    }

    println!("{:?}", array);
}