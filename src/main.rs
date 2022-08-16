// write a program that rolls a dice ten times.

fn main() {
    let mut roll_count = 0;
    let mut roll_total = 0;
    while roll_count < 10 {
        roll_total += rand::thread_rng().gen_range(1, 7);
        roll_count += 1;
    }
    println!("Total: {}", roll_total);
}