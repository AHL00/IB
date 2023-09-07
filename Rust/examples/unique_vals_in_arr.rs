use painless_input::input_array;

fn main() {
    let arr: Vec<i32> = input_array("Enter array: ");
    println!();

    // Whenever number found, go to start of array and iterate, if number found, break
    // If number not found, add to count
    // This should be more efficient than another array and having to search the other array every time

    let mut count = 0;

    for i in 0..arr.len() {
        let mut found = false;

        // Only iterate up to i, because we don't need to check the numbers after i
        for j in 0..i {
            if arr[i] == arr[j] {
                found = true;
                break;
            }
        }

        if !found {
            count += 1;
        }
    }

    println!("Number of unique values: {}", count);
}