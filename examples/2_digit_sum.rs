use ib::input;

fn main() {
    let num: i32 = input("Input a number: ");

    let sum = num % 10 + num / 10;
    println!("Sum: {}", sum);
}