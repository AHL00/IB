use rand;

fn main() {
    // Array with 10 random numbers
    let mut arr: [i16; 10] = [0; 10];

    // Fill the array with random numbers
    for i in 0..10 {
        arr[i] = rand::random();
    }

    // Print the array
    println!("{:?}", arr);
}