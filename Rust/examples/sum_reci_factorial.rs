use ib::input;

fn main() {
    let n = input::<i32>("Input n: ");

    let mut sum = 0.0;
    let mut factorial = 1;

    for i in 1..(n + 1) {
        factorial *= i;
        sum += 1.0 / factorial as f64;
    }

    println!("Sum: {}", sum);
}