use ib::input;

fn main() {
    // Distance between two points
    // (x1, y1) and (x2, y2)

    println!("Input two coordinates");

    let x1 = input::<f32>("x1: ");
    let y1 = input::<f32>("y1: ");
    let y2 = input::<f32>("y2: ");
    let x2 = input::<f32>("x2: ");

    // Formula: sqrt((x2 - x1)^2 + (y2 - y1)^2)
    let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    println!("Distance: {}", distance);
}
