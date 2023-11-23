fn main() {
    let a: [i32; 10] = rand::random(); // Generate random array
    let mut b = [0; 10]; // Empty array of length 10

    let mut b_cursor = 0;

    for num in a {
        if num % 2 != 0 {
            b[b_cursor] = num;
            b_cursor += 1;
        }
    }

    println!("Array: {:?}", a);
    println!("Odd numbers: {:?}", b);
}