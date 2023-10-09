use painless_input;

fn main() {
    let num: i32 = painless_input::input("Input a number: ");
    println!();

    let mut divisor_sum = 0;

    for i in 1..num {
        if num % i == 0 {
            divisor_sum += i;
        }
    }

    println!("{} is {}", num, match divisor_sum {
        sum if sum == num => "perfect",
        sum if sum > num => "abundant",
        sum if sum < num => "deficient",
        _ => "invalid"
    });
}