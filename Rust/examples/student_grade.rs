// >75 = A
// >50 = B
// >25 = C
// >0 = D

use painless_input::input;

fn main() {
    let score = input::<f32>("Input a score: ");

    let grade = if score > 75.0 {
        "A"
    } else if score > 50.0 {
        "B"
    } else if score > 25.0 {
        "C"
    } else {
        "D"
    };

    println!("Grade: {}", grade);
}