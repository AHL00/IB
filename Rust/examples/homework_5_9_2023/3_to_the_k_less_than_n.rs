// 3^k > n
// Smallest k

use ib::input;

fn main() {
    let n = input::<u128>("Input n: ");

    let mut k = 0;
    let mut three_to_the_k = 1;

    while three_to_the_k < n {
        three_to_the_k *= 3;
        k += 1;
    }

    println!("3^{} = {}", k, three_to_the_k);
}