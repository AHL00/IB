// Check if a given array is sorted, either ascending or descending.

fn main() {
    let arr= [12, 34, 56, 78, 90, 87, 65, 43, 21];

    println!("\nTesting: {:?}\n", arr);


    let mut is_ascending = true;
    let mut is_descending = true;

    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            is_ascending = false;
        } else if arr[i] < arr[i + 1] {
            is_descending = false;
        }
    }

    println!("Is ascending: {}", is_ascending);
    println!("Is descending: {}", is_descending);
}
