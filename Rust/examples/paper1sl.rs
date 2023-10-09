use painless_input;

fn main() {
    let students: Vec<String> = painless_input::input_array("Enter students: ");
    println!();

    let scores: Vec<i32> = painless_input::input_array("Enter scores: ");
    println!();
    println!();

    let mut grades = vec![];

    for (i, name) in students.iter().enumerate() {
        println!("{}: {}", name, scores[i]);

        let grade = match scores[i] {
            80..=100 =>"Distinction",
            60..=79 => "Merit",
            40..=59 => "Pass",
            0..=39 => "Fail",
            _ => "Invalid score",
        };

        println!("Grade: {}\n", grade);

        grades.push(grade);
    }

    println!();

    // Which grade would you like to display?
    let grade_options = ["Distinction", "Merit", "Pass", "Fail"];
    let display_grades = painless_input::multiselect_input("Which grade would you like to display?", "Done", &grade_options);
    println!();
    println!();

    // display grades is Vec<bool>
    let display_grades: Vec<&str> = display_grades.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| grade_options[i]).collect();

    for (i, grade) in grades.iter().enumerate() {
        if display_grades.contains(&grade) {
            println!("{}: {}", students[i], grade);
        }
    }
}