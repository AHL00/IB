fn main() {
    // 3x3 grid
    // Each row, column, diagonal should add to 15 and must have 1-9 non repeating

    let mut threads = vec![];

    for i in 0..9 {
        threads.push(std::thread::spawn(move || {
            thread(i);
        }));
    }

    let start = std::time::Instant::now();

    for thread in threads {
        thread.join().unwrap();
    }

    let end = std::time::Instant::now();

    println!("Completed in {:?}", (end - start));
}

fn thread(first_cell: i8) {
    let mut grid = [0u8; 9];

    println!("Thread {} started", first_cell);

    // Cell 0, 0
    grid[0] = first_cell as u8 + 1;

    for i in 0..9 {
        // Cell 1, 0
        grid[1] = i as u8 + 1;

        for j in 0..9 {
            // Cell 2, 0
            grid[2] = j as u8 + 1;

            for k in 0..9 {
                // Cell 0, 1
                grid[3] = k as u8 + 1;

                // Cell 1,1 is always 5
                grid[4] = 5;

                for m in 0..9 {
                    // Cell 2, 1
                    grid[5] = m as u8 + 1;

                    for n in 0..9 {
                        // Cell 0, 2
                        grid[6] = n as u8 + 1;

                        for o in 0..9 {
                            // Cell 1, 2
                            grid[7] = o as u8 + 1;

                            for p in 0..9 {
                                // Cell 2, 2
                                grid[8] = p as u8 + 1;

                                if is_valid(&grid) {
                                    println!("{:?}\n", grid);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_valid(grid: &[u8; 9]) -> bool {
    // Check rows
    for i in 0..3 {
        let mut sum = 0;

        for j in 0..3 {
            sum += grid[i * 3 + j];
        }

        if sum != 15 {
            return false;
        }
    }

    // Check columns
    for i in 0..3 {
        let mut sum = 0;

        for j in 0..3 {
            sum += grid[i + j * 3];
        }

        if sum != 15 {
            return false;
        }
    }

    // Check diagonals
    if grid[0] + grid[4] + grid[8] != 15 {
        return false;
    }

    if grid[2] + grid[4] + grid[6] != 15 {
        return false;
    }

    true
}
