static mut FIBONACCI: i32 = 0;

fn main() {
    let start = std::time::Instant::now();
    println!("{}", fibonacci(45));
    println!("Time: {:?}", start.elapsed());
    unsafe { println!("Calls: {}", FIBONACCI) }
}


fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    unsafe {
        FIBONACCI += 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}