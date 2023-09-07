use painless_input::input;

fn main() {
    let mut num: i32 = input("Input a number: ");

    let mut reversed: i32 = 0;

    loop {
        // * 10 to left shift
        // + num % 10 to add the digit to the newly emoty space on the right
        // For example, start with 123, after one iter we have 12 left in var
        // 12 % 10 = 2
        // After left shift, reversed var which is 3 is now 30.
        // The + num % 10 adds the 2 to the right of 30, so now we have 32.
        // On and on until num is 0, then we break out of the loop.
        reversed = reversed * 10 + num % 10;

        // Remove last digit from num
        num /= 10;

        // Break out of loop if num is empty
        if num == 0 {
            break;
        }
    }

    println!("Reversed: {}", reversed);
}