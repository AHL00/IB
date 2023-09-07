use painless_input::input;

fn main() {
    let square_side = input::<f32>("Enter the side of the square: ");
    let rect_side_a = input::<f32>("Enter the side A of the rectangle: ");
    let rect_side_b = input::<f32>("Enter the side B of the rectangle: ");

    let height_squares = (rect_side_a / square_side) as i32;
    let width_squares = (rect_side_b / square_side) as i32;

    println!("The rectangle can contain {} squares", height_squares * width_squares);

    let num_squares = height_squares * width_squares;
    
    let area_squares = square_side * square_side * num_squares as f32;

    println!("The area of the squares is {}", area_squares);

    let area_rect = rect_side_a * rect_side_b;

    println!("The area of the rectangle is {}", area_rect);

    let remainder_area = area_rect - area_squares;

    println!("The remainder area is {}", remainder_area);
}