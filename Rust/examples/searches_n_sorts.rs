use std::time::Instant;
use rand;

fn main() {
    let mut array: Box<[i16]> = vec![0; 100000].into_boxed_slice();

    for i in 0..array.len() {
        array[i] = rand::random();
    }

    println!("Random generated");

    let start_time = Instant::now(); // Start measuring time

    let mut bubble_array = array.clone();
    bubble_sort(bubble_array.as_mut());
    drop(bubble_array); // Drop bubble_array to free up memory (not necessary

    let end_time = Instant::now(); // Stop measuring time

    println!("Bubble sort, Elapsed time: {:?}", end_time - start_time);

    let start_time = Instant::now(); // Start measuring time

    let mut selection_array = array.clone();
    selection_sort(selection_array.as_mut());
    drop(selection_array); // Drop selection_array to free up memory (not necessary

    let end_time = Instant::now(); // Stop measuring time

    println!("Selection sort, Elapsed time: {:?}", end_time - start_time);

    // Generate search num
    // Random from 0 to array.len
    let mut sorted_array = array.clone();
    sorted_array.sort();
    drop(array); // Drop array to free up memory (not necessary)

    let search_idx = rand::random::<usize>() % sorted_array.len();
    let search_num = sorted_array[search_idx];

    println!("Searching for {} which is at index {}", search_num, search_idx);

    let start_time = Instant::now(); // Start measuring time

    let linear_res = linear_search(sorted_array.as_ref(), search_num).unwrap();

    let end_time = Instant::now(); // Stop measuring time

    println!("Linear search found at: {}, Elapsed time: {:?}", linear_res, end_time - start_time);

    let start_time = Instant::now(); // Start measuring time

    let binary_res = binary_search(sorted_array.as_ref(), search_num).unwrap();

    let end_time = Instant::now(); // Stop measuring time

    println!("Binary search found at: {}, Elapsed time: {:?}", binary_res, end_time - start_time);
}

fn linear_search(arr: &[i16], n: i16) -> Option<usize> {
    for (i, num) in arr.iter().enumerate() {
        if *num == n {
            return Some(i);
        }
    }

    None
}

fn binary_search(sorted_arr: &[i16], n: i16) -> Option<usize> {
    let mut left = 0;
    let mut right = sorted_arr.len();

    while left <= right {
        let mid = (left + right) / 2;

        if n == sorted_arr[mid] {
            return Some(mid);
        }

        if n > sorted_arr[mid] {
            left = mid;
        } else {
            right = mid;
        }
    }

    None
}

fn bubble_sort(arr: &mut [i16]) {
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

fn selection_sort(arr: &mut [i16]) {
    for i in 0..arr.len() {
        let mut min_idx = 0;

        for j in i..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        arr.swap(i, min_idx);
    }
}