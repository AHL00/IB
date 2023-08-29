use ib::input;

fn main() {
    let num = input::<i32>("Input a number: ");

    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}