// Check if a given array is sorted, either ascending or descending.
use ib::input;

fn main() {
    let mut arr;

    println!("Input test case: \n 1. Ascending \n 2. Descending \n 3. Random");

    let test_case: i32 = input("Enter test case: ");

    match test_case {
        1 => arr = [1, 2, 3, 4, 5],
        2 => arr = [5, 4, 3, 2, 1],
        3 => {
            arr = [0; 5];

            for i in 0..5 {
                arr[i] = rand::random::<i16>() as i32;
            }
        },
        _ => panic!("Invalid test case"),
    }

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
