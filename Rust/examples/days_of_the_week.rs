use ib::input;

fn main() {
    // The third day of the week is 1st January. 
    // What day is the 100th day after

    let start_day = input::<i32>("Enter starting day of the week (1-7): ");

    let days = input::<i32>("Enter number of days: ");

    let remainder = days % 7;

    let end_day_of_week = start_day + remainder;

    let weeks_overflow = (end_day_of_week / 7) as i32;

    let end_day_of_week = end_day_of_week - weeks_overflow * 7;

    println!("The day after {} days is {}", days, end_day_of_week);
}
