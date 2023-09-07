// 500 runners
// Running time recorded in array of 500 ints as seconds
// First 5 runners is in one team, next 5 in another, and so on
// Best 3 is taken as final score

fn main() {
    // Test data for 5 teams
    let arr: [i16; 25] = [
        2400, 2410, 1200, 14003, 14250, 
        2395, 2408, 21905, 24220, 24350,
        2390, 2403, 2418, 2427, 2432, 
        2405, 2412, 2423, 2434, 2413,
        2395, 2400, 2410, 2402, 2408
    ]; 

    let mut first_team_time = i32::MAX;
    let mut first_team_num = 0;
    
    let mut second_team_time = i32::MAX;
    let mut second_team_num = 0;

    let mut third_team_time = i32::MAX;
    let mut third_team_num = 0;

    for i in 0..25 {
        if i % 5 != 0 {
            continue;
        }

        let mut first = 0;
        let mut second = 0;
        let mut third = 0;

        for j in i..i + 5 {
            if arr[j] > first {
                third = second;
                second = first;
                first = arr[j];
            } else if arr[j] > second {
                third = second;
                second = arr[j];
            } else if arr[j] > third {
                third = arr[j];
            }
        }

        let team_time: i32 = first as i32 + second as i32 + third as i32;

        if team_time < first_team_time {
            third_team_time = second_team_time;
            third_team_num = second_team_num;

            second_team_time = first_team_time;
            second_team_num = first_team_num;

            first_team_time = team_time;
            first_team_num = (i / 5) + 1;
        } else if team_time < second_team_time {
            third_team_time = second_team_time;
            third_team_num = second_team_num;

            second_team_time = team_time;
            second_team_num = (i / 5) + 1;
        } else if team_time < third_team_time {
            third_team_time = team_time;
            third_team_num = (i / 5) + 1;
        }


        println!("Team {}: {} {} {}", (i / 5) + 1, first, second, third);
    }

    println!("First: [ Team {} ] {} seconds", first_team_num, first_team_time);
    println!("Second: [ Team {} ] {} seconds", second_team_num, second_team_time);
    println!("Third: [ Team {} ] {} seconds", third_team_num, third_team_time);
}  