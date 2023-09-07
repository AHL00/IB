use painless_input::input_array;

fn main() {
    let arr: Vec<i32> = input_array("Enter array: ");
    println!();

    let mut is_alternating = true;
    let first_sign = check_sign(arr[0]);

    for i in 1..arr.len() {
        // If the sign matches the first sign and the index is odd, then it is not alternating
        // If the sign does not match the first sign and the index is even, then it is not alternating
        if check_sign(arr[i]) == first_sign && i % 2 == 1
        || check_sign(arr[i]) != first_sign && i % 2 == 0
        {
            is_alternating = false;
            break;
        }
    }
    
    if is_alternating {
        println!("The array is alternating.");
    } else {
        println!("The array is not alternating.");
    }
}

fn check_sign(num: i32) -> bool {
    if num < 0 {
        false
    } else {
        true
    }
}
