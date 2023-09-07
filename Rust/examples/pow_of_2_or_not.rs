use painless_input::input;

// I know this is not the smallest version of the code but I added
// edge cases to make it more efficient in some cases
fn main() {
    let num = input::<i32>("Input a number: ");

    let mut result = false;

    // If even, go on
    if num % 2 == 0 {
        let mut num = num;

        // Divide by 2 until less than 1
        while num > 1 {
            num /= 2;

            if num % 2 != 0 {
                // If remainder is not 0, then it's not a power of 2
                break;
            }
        }

        // If 1, then it's a power of 2
        if num == 1 {
            result = true;
        }
    }

    println!("Result: {}", result);
}