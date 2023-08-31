// Homework for 31/8/2023
// Given the coords of two points on a chess board, a knight can perform the move

use ib::input;

fn main() {
    let (x1, y1) = (
        input::<i32>("Enter the first x coord: "),
        input::<i32>("Enter the first y coord: "),
    );

    let (x2, y2) = (
        input::<i32>("Enter the second x coord: "),
        input::<i32>("Enter the second y coord: "),
    );

    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();

    let possible = (dx == 1 && dy == 2) || (dx == 2 && dy == 1);

    println!("The move is possible: {}", possible);
}