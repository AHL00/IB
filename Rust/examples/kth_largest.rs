type Data = i32;

/// "Cascade" this number down, swapping until we find a None or a number that is smaller
/// If we get to the end, that means this number is the smallest of the top k and gets replaced
fn cascade_down(arr: &mut Vec<Option<Data>>, start_idx: usize, mut n: Data) {
    let mut i = start_idx;

    while i < arr.len() {
        // If the element is None, replace it
        if arr[i].is_none() {
            arr[i] = Some(n);
            break;
        }

        // If the element is smaller than n, replace it and cascade it down
        if arr[i].unwrap() < n {
            let replaced = arr[i];
            arr[i] = Some(n);
            n = replaced.unwrap();
        }

        i += 1;
    }
}

fn main() {
    let data_str = include_str!("../../5000_random.txt");

    let mut arr: [Data; 5000] = [0; 5000];

    for (i, line) in data_str.lines().enumerate() {
        arr[i] = line.strip_suffix(',').unwrap().parse().unwrap();
    }

    let k = 4000;

    let mut top_k_vec: Vec<Option<Data>> = vec![None; k];

    /* Start timing */
    let start = std::time::Instant::now();

    for n in arr {
        cascade_down(&mut top_k_vec, 0, n);
    }

    let elapsed = start.elapsed();

    println!("Top {} element: {:?}", k, top_k_vec.last());

    println!("Time taken: {:?}", elapsed);
}


        // for (i, &m) in top_k_vec.iter().enumerate() {
        //     if let Some(m) = m {
        //         if m < n {
        //             idx = Some(i);
        //             break;
        //         }
        //     } else {
        //         // If None, replace it
        //         idx = Some(i);
        //         break;
        //     }
        // }

        // if let Some(i) = idx {
        //     // println!("Swapping {} with {:?}", n, top_k_vec[i]);
        //     let replaced = top_k_vec[i];
        //     top_k_vec[i] = Some(n);

            
        //     if let Some(replaced) = replaced {
        //         cascade_down(&mut top_k_vec, i, replaced);
        //     }
        // }