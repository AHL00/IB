use painless_input::input;

fn main() {
    // Use Heron's formula to calculate the area of a triangle

    let x1 = input::<f32>("x1: ");
    let y1 = input::<f32>("y1: ");

    let x2 = input::<f32>("x2: ");
    let y2 = input::<f32>("y2: ");

    let x3 = input::<f32>("x3: ");
    let y3 = input::<f32>("y3: ");

    // Formula: sqrt(s * (s - a) * (s - b) * (s - c))

    let a = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    let b = ((x3 - x2).powi(2) + (y3 - y2).powi(2)).sqrt();
    let c = ((x1 - x3).powi(2) + (y1 - y3).powi(2)).sqrt();

    let s = (a + b + c) / 2.0;

    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

    println!("Area: {}", area);
}