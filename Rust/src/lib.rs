pub fn input<T>(input_str: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Get data from user
    // and convert it to float
    println!("{}", input_str);

    let res: T;

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_res = input.trim().parse();

        if input_res.is_ok() {
            res = input_res.unwrap();
            break;
        } else {
            println!("Invalid input, try again");
        }
    }

    res
}

pub fn input_array<T>(input_str: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Get data from user
    // and convert it to float
    println!("{}", input_str);

    let mut res: Vec<T> = Vec::new();

    // print [ and wait for user input
    println!("Press enter to finish");

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "" {
            break;
        }

        let input_res = input.trim().parse();

        if input_res.is_ok() {
            res.push(input_res.unwrap());
        } else {
            println!("Invalid input, try again");
        }
    }

    res
}
