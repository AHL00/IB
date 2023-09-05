// Bruteforce is prime

use std::thread::JoinHandle;

use ib::input;

// Benches
// Starting threads takes 4 ms

// 5915587277 -> 4 ms
// 27292996856087 -> 9 ms

fn main() {
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

