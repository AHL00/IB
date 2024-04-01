use painless_input::input;
use std::{thread::JoinHandle, time::Duration};

fn prime() {
    let n = input::<u128>("Input n: ");
    let num_threads = input::<u128>("Input number of threads: ");

    let mut is_prime = true;

    let limit = (n as f64).sqrt() as u128;

    println!("Limit: {}", limit);

    let start = std::time::Instant::now();

    // Every thread will get a range of the n to check
    // If a factor is found, it will return it
    let mut threads: Vec<JoinHandle<u128>> = vec![];
    let range = limit / num_threads;

    for i in 0..num_threads {
        let mut start = i * range;
        let mut end = (i + 1) * range;

        // Make sure the first thread doesn't attempt to divide by 0
        if i == 0 {
            start = 2;
        }

        if i == num_threads - 1 {
            end = limit;
        }

        println!("Starting thread {} [{} to {}]", i, start, end);

        threads.push(std::thread::spawn(move || {
            for i in start..end + 1 {
                if n % i == 0 {
                    return i;
                }
            }

            return 0;
        }));
    }

    for thread in threads {
        let factor = thread.join().unwrap();

        if factor != 0 {
            println!("Factor found: {}", factor);
            is_prime = false;
            break;
        }
    }

    let end = std::time::Instant::now();

    println!("Time taken: {:?}", end - start);

    println!("Is prime: {}", is_prime);

    // Test: 101103107109113127131137139149151157163

    // Input to not stop program
    input::<String>("");
}

fn next_prime(n: u32) -> u32 {
    let mut n = n + 1;

    loop {
        let limit = (n as f64).sqrt().ceil() as u32;

        let mut is_prime = true;
        let mut ran = false;

        for i in 2..limit {
            ran = true;
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime && ran || n == 2 || n == 3 {
            return n;
        }

        n += 1;
    }
}

#[test]
fn test_next_prime() {
    assert_eq!(next_prime(1), 2);
    assert_eq!(next_prime(2), 3);
    assert_eq!(next_prime(3), 5);
    assert_eq!(next_prime(4), 5);
    assert_eq!(next_prime(5), 7);
}

fn main() {
    let mut p: u32 = 0;
    let mut threads: Vec<JoinHandle<(u128, bool)>> = vec![];

    let mut maxed = false;

    loop {
        for i in 0..1 {
            if threads.len() > i {
                // Check current thread for status
                if threads[i].is_finished() {
                    let res = threads.remove(i).join().unwrap();

                    println!("{}: {}", res.0, res.1);
                }
            } else {
                if !maxed {
                    // Calc next candidate
                    p = next_prime(p);

                    let mersenne = (2u128 << (p - 2));

                    let candidate = mersenne.saturating_mul((2u128 << (p - 1)) - 1);
    
                    if candidate == u128::MAX {
                        maxed = true;
                        println!("Maxed out");
                    }
                    
                    // println!("Checking: {}", candidate);
    
                    // Add a new thread
                    threads.push(std::thread::spawn(move || thread(candidate)));
                }
            }
        }
    }
}

fn thread(n: u128) -> (u128, bool) {
    let mut sum: u128 = 0;

    let mut last_divisor = 0;
    for i in 2..n {
        if n % i == 0 {
            let div_1 = i;
            let div_2 = n / i;

            if div_2 == last_divisor {
                break;
            }

            sum += div_1 + div_2;

            last_divisor = div_1;
        }
    }

    sum += 1;

    if sum == n {
        return (n, true)
    }

    (n, false)    
}