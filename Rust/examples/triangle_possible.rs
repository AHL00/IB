// Homework for 31/8/2023
// Triangle possible given three sides?

use painless_input::input;

fn main() {
    let a = input::<i32>("Side a: ");
    let b = input::<i32>("Side b: ");
    let c = input::<i32>("Side c: ");

    let possible = a + b > c && a + c > b && b + c > a;

    println!("Triangle possible: {}", possible);

    input::<String>("Press any key to exit");
}