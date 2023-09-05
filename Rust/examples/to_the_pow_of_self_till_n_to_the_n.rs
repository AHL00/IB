// 1^1 + 2^2 + ... + n^n

use ib::input;

fn main() {
    let n = input::<i32>("Input n: ");

    let mut sum = 0;

    for i in 1..n + 1 {
        let mut term = i;
        
        for _ in 0..i - 1 {
            term *= i;
        }

        sum += term;
    }

    println!("Sum: {}", sum);
}