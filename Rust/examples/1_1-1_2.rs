// 1.1 - 1.2 + 1.3 - 1.4

use painless_input::input;

fn main() {
    let n = input::<i32>("Input n: ");

    let mut sum = 0.0;

    for i in 1..n + 1 {
        let term = 1.0 + 0.1 * i as f64;

        if i % 2 == 0 {
            sum -= term;
        } else {
            sum += term;
        }
    }

    println!("Sum: {:.2}", sum);
}
