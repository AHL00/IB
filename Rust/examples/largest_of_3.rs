// Given A, B, C, find the largest

use painless_input::input;

fn main() {
    let a = input::<i32>("Input a: ");
    let b = input::<i32>("Input b: ");
    let c = input::<i32>("Input c: ");

    // Find the largest
    let largest = if a > b {
        if a > c {
            a
        } else {
            c
        }
    } else {
        if b > c {
            b
        } else {
            c
        }
    };

    println!("Largest: {}", largest);

    // Find the 2 largest
    let smallest = if a < b {
        if a < c {
            a
        } else {
            c
        }
    } else {
        if b < c {
            b
        } else {
            c
        }
    };

    let total_two_largest = a + b + c - smallest;

    println!("Total of two largest: {}", total_two_largest);
}