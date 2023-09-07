use rand;

fn main() {
    // Array with 10 random numbers
    let mut arr: [i16; 10] = [0; 10];

    // Fill the array with random numbers
    for i in 0..10 {
        arr[i] = rand::random();
    }

    // Find largest
    let mut largest = 0;

    for i in 0..10 {
        if arr[i] > largest {
            largest = arr[i];
        }
    }

    // Print the array
    println!("{:?}", arr);

    // Print the largest
    println!("Largest: {}", largest);
}