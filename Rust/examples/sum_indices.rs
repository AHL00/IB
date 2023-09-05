use ib::input;

fn main() {
    // A^0 + A^1 + A^2 + ... + A^n

    let a = input::<f64>("Input a: ");
    let n = input::<i32>("Input n: ");

    // start sum at 1.0 because A^0 = 1
    let mut sum = 1.0;

    // this gets multiplied by A each time
    let mut current_sum_term = 1.0;
   
    // start at 1 because we already have A^0
    for _ in 1..(n + 1) {
        current_sum_term *= a;
        sum += current_sum_term;
    }

    println!("Sum: {}", sum);
}