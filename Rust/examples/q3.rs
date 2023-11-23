fn main() {
    let mut last_fib = 1;
    let mut this_fib = 1;
    let mut n = 2;

    loop {
        let next_fib = last_fib + this_fib;
        last_fib = this_fib;
        this_fib = next_fib;
        n += 1;

        if this_fib > 999 {
            println!("Smallest fibonacci number with 4 digits found at index {}: {}", n, this_fib);
            break;
        }

    }
}