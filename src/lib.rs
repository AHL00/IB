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
