use painless_input::input;

fn main() {
    let num = input::<i32>("Input a number: ");

    println!("It is odd: {}", (num % 2 != 0) as bool);
}