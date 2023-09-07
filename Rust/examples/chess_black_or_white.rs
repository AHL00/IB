use painless_input::input;

fn main() {
    // (1, 1) is black

    let (x, y) = (input::<i32>("X coord: "), input::<i32>("Y coord: "));

    // let x_black = x % 2 == 1;
    // let y_black = y % 2 == 1;

    // let black = x_black == y_black;

    let black = (x + y) % 2 == 0;

    println!("The square is black: {}", black);

}