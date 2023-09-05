use ib::input;

fn main() {
    let mut num: i32 = input("Input a number: ");
    let mut sum = 0;

    while (num > 0) {
        sum += num % 10;
        num /= 10;
    }

    println!("Sum: {}", sum);
}