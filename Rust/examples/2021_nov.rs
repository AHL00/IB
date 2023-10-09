use painless_input;

static STATIONS: [&str; 10] = [
    "Oppox",
    "Thamesley",
    "Brinkley",
    "Kiko",
    "Endsley",
    "Kingsley",
    "Allapay",
    "Kronos",
    "Longlines",
    "Dovely"
];

static DISTANCES: [f32; 10] = [
    0.0,
    1.2,
    1.0,
    2.2,
    1.3,
    1.4,
    0.9,
    1.1,
    1.2,
    0.9
];

fn main() {
    let s1 = STATIONS[painless_input::select_input("Enter station 1: ", &STATIONS)];
    println!();
    let s2 = STATIONS[painless_input::select_input("Enter station 2: ", &STATIONS)];
    println!();

    let distance = calculate_distance(s1, s2);

    let age: i32 = painless_input::input("Enter your age: ");
    println!();

    let rate = 0.2;

    let mut price = distance * rate;

    price = match age {
        0..=4 => price * 0.0,
        6..=15 => price * 0.5,
        16..=65 => price * 1.0,
        66.. => price * 0.65,
        _ => price
    };

    println!();
    println!("Distance: {}", distance);
    println!("Price: ${:.2}", price);
}

fn calculate_distance(s1: &str, s2: &str) -> f32 {
    let mut found = false;
    let mut distance = 0.0;

    let mut last_station = "";

    for (i, station) in STATIONS.iter().enumerate() {
        if station == &s1 {
            found = true;
            if last_station == s2 {
                distance += DISTANCES[i];
                break;
            }
            else if last_station == "" {
                last_station = s1;
            }
        } else if station == &s2 {
            found = true;
            if last_station == s1 {
                distance += DISTANCES[i];
                break;
            }
            else if last_station == "" {
                last_station = s2;
            }
        } else if found {
            distance += DISTANCES[i];
        }
    }

    distance
}