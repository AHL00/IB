// Given A and D, fill an array with N terms
use ib::input;

fn main() {
    let a: i32 = input("Enter A: ");
    let d: i32 = input("Enter D: ");
    let n: i32 = input("Enter N: ");

    let mut arr = Vec::new();

    for i in 0..n {
        arr.push(a + i * d);
    }

    println!("{:?}", arr);
}