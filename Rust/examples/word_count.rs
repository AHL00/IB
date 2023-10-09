use painless_input::input;

fn main() {
    let input_sentence: String = input("Enter a sentence: ");
    println!();

    let mut word_count = 1;

    for (i , char) in input_sentence.chars().enumerate() {
        if char == ' ' {
            word_count += 1;
        }
    }

    println!("Word count: {}", word_count);
}