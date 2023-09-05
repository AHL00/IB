// 1^k + 2^k + ... + n^k

use ib::input;

fn main() {
    let n = input::<i32>("Input n: ");
    let k = input::<i32>("Input k: ");

    let mut sum = 0;

    for i in 1..n + 1 {
        let mut term = i;
        
        for _ in 0..k - 1 {
            term *= i;
        }

        sum += term;
    }

    println!("Sum: {}", sum);
}