use painless_input::input;

fn main() {
    let mut number = input::<i32>("Input a number: ");

    let original = number;
    let mut reversed = 0;

    while number > 0 {
        // Move the last digit of the number to the last digit of reversed
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }

    println!("{} is {} palindrome", original, if original == reversed { "a" } else { "not a" });
}