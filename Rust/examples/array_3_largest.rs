use rand;

fn main() {
    // Array with 10 random numbers
    let mut arr: [i16; 10] = [0; 10];

    // Fill the array with random numbers
    for i in 0..10 {
        arr[i] = rand::random();
    }

    // Find 3 largest
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    for i in 0..10 {
        if arr[i] > first {
            third = second;
            second = first;
            first = arr[i];
        } else if arr[i] > second {
            third = second;
            second = arr[i];
        } else if arr[i] > third {
            third = arr[i];
        }
    }

    // Print the array
    println!("{:?}", arr);
    
    // Print the largest
    println!("Largest: {}", first);
    println!("Second largest: {}", second);
    println!("Third largest: {}", third);
}