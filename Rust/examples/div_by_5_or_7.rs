// Homework for 31/8/2023
// Check if a number is divisible by 5 or 7

use ib::input;

fn main() {
    let num = input::<i32>("Enter a number: ");

    let divisible = num % 5 == 0 || num % 7 == 0;

    println!("{} is divisible by 5 or 7: {}", num, divisible);
}