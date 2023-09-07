use painless_input::input;

fn main() {
    // Find the fibonacci and print each term up to n
    let n = input::<i32>("Input n: ");

    println!("Fibonacci up to {}:", n);
    println!();

    let mut current_term = 1;
    let mut last_term = 0;

    for i in 0..n {
        println!("[{}] {}", i, current_term);

        let temp = current_term;
        current_term += last_term;
        last_term = temp;
    }

    println!();

    println!("Fibonacci recursive up to {}:", n);
    println!();
    println!("{}", fibonacci_recursive(n));
}

fn fibonacci_recursive(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}