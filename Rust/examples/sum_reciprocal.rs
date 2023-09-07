use painless_input::input;

fn main() {
    let n = input::<i32>("Input n: ");

    let mut sum = 0.0;

    for i in 1..(n + 1) {
        sum += 1.0 / i as f64;
    }

    println!("Sum: {}", sum);
}