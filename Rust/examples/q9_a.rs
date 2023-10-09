use painless_input;

// [0] is for Mon to Fri, [1] is for Sat to Sun
static FEES: [u16; 2] = [20, 30];

fn main() {
    let data: [u16; 21] = [
        452, 231, 951, 672, 102,  302, 523,
        912, 123, 845, 429, 942, 583, 732,
        392, 582, 694, 283, 915, 483, 721
    ];

    let mut two_d_data:  [[u16; 7]; 3] = [[0; 7]; 3];

    for i in 0..data.len() {
        two_d_data[i / 7][i % 7] = data[i];
    }

    let mut highest_avg = 0;
    let mut highest_avg_idx = 0;

    for day_idx in 0..7 {
        let column_total = total(day_idx, &two_d_data);

        if column_total > highest_avg {
            highest_avg = column_total / 3;
            highest_avg_idx = day_idx;
        }
    }

    println!("The highest average is {} on {}.", highest_avg, convert(highest_avg_idx));

    let start_date = (0, 1);
    let end_date = (1, 1);

    let sales_total = salesCalculate(start_date, end_date, &two_d_data);

    println!("The total sales from {} to {} is ${}.", convert(start_date.0), convert(end_date.0), sales_total);
}

fn salesCalculate(mut start_date: (usize, usize), mut end_date: (usize, usize), data: &[[u16; 7]]) -> u32 {
    let mut total: u32 = 0;

    for i in start_date.0..=end_date.0 {
        for j in start_date.1..=end_date.1 {
            let fee = if i >= 5 { FEES[1] } else { FEES[0] };
            total += data[j][i] as u32 * fee as u32;
        }
    }

    total
}

fn convert(day: usize) -> &'static str {
    return match day {
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Saturday",
        6 => "Sunday",
        _ => "Invalid"
    };
}

fn total(column: usize, data: &[[u16; 7]]) -> u16 {
    let mut total = 0;

    for row in data {
        total += row[column];
    }

    return total;
}