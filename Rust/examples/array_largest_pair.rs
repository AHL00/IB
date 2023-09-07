use rand;

fn main() {
    // Array with 10 random numbers
    let mut arr: [i16; 10] = [0; 10];

    // Fill the array with random numbers
    for i in 0..10 {
        arr[i] = rand::random();
    }

    // Find largest pair
    let mut largest_pair_sum: i32 = 0;

    for i in 0..arr.len() - 1 {
        let sum = arr[i] as i32 + arr[i + 1] as i32;

        if sum > largest_pair_sum {
            largest_pair_sum = sum as i32;
        }
    }

    // Print the array
    println!("{:?}", arr);
    
    // Print the largest
    println!("Largest pair sum: {}", largest_pair_sum);
}