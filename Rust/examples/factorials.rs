// Given N, find N factorial

use ib::input;

fn main() {
    let mut start = 1;

    let n = input::<i32>("Input n: ");

    for i in 1..(n + 1) {
        start *= i;
    }

    println!("{}! = {}", n, start);
}
