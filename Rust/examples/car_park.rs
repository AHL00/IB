
struct CarParkRecord {
    id: usize,
    /// Unix timestamp
    arrival: u32,
    /// Unix timestamp
    paid_at: u32,
}

// Up to 30 mins, Up to 2 hours, Max daily
static RULES: [[f32; 3]; 2] = [
    // Fees
    [0.5, 3.0, 0.0],
    // Fines
    [5.0, 15.0, 30.0],
];

fn calculate_cost(record: &CarParkRecord) -> f32 {
    let stayed_for = record.paid_at - record.arrival;

    let mut fees = 0.0;

    let mut overtime = 0;

    if stayed_for <= 30 * 60 {
        fees = RULES[0][0];
        overtime = 0;
    } else if stayed_for <= 120 * 60 {
        fees = RULES[0][1];
        overtime = 0;
    } else {
        
        let days = stayed_for / 86400;

        if days == 0 {
            overtime = stayed_for - 86400;
        } else {
            overtime = stayed_for - (86400 * days);
        }

        fees = days as f32 * RULES[0][2];

    }

    let mut fines = 0.0;

    // if stayed_for > 

    fees + fines
}

fn main() {
    panic!("This example is not meant to be run");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    const durations: [u32; 4] = [29 * 60, 40 * 60, 130 * 60, 86400 * 2]; 

    #[test]
    fn test_all_durations() {
        for duration in durations.iter() {
            let duration_ = Duration::from_secs(*duration as u64);

            let record = CarParkRecord {
                id: 1,
                arrival: 0,
                paid_at: *duration,
            };

            println!("[{:?}] Cost: {}", duration_, calculate_cost(&record));
        }
    }
}
