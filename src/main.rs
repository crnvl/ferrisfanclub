/* 
    create a program that rolls a dice 10 times and checks if the sum is even
*/

use rand::{Rng}; // 0.8.0

fn main() {
    let mut sum = 0;
    for i in 0..10 {
        let dice = rand::thread_rng().gen_range(1..7);
        sum += dice;
    }
    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}