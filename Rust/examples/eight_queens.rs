// 8 queens not overlapping
fn main() {
    check(&[28]);

    let mut solutions = vec![];

    'main: for i in 0..64 {
        for j in i + 1..64 {
            for k in j + 1..64 {
                for l in k + 1..64 {
                    for m in l + 1..64 {
                        for n in m + 1..64 {
                            for o in n + 1..64 {
                                for p in o + 1..64 {
                                    let queens = [i, j, k, l, m, n, o, p];

                                    if check(&queens) {
                                        solutions.push(queens);
                                        break 'main;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Done");
    println!("Solutions: {:?}", solutions.len());
    println!("First solution: {:?}", solutions[0]);  

}

fn check(queens: &[u8]) -> bool {
    let mut board = [false; 64];

    for queen in queens.iter() {
        // If the queen's pos is already true, then we have a collision
        if board[*queen as usize] {
            return false;
        }

        let hor = *queen as usize % 8;
        let ver = *queen as usize / 8;

        // Check straights
        for i in 0..8 {
            board[hor + i * 8] = true;
            board[ver * 8 + i] = true;
        }

        // Check diagonals
        for i in 0..8 {
            if hor + i < 8 && ver + i < 8 {
                board[hor + i + (ver + i) * 8] = true;
            }

            if hor + i < 8 && ver as i8 - i as i8 >= 0 {
                board[hor + i + (ver - i) * 8] = true;
            }

            if hor as i8 - i as i8 >= 0 && ver + i < 8 {
                board[hor - i + (ver + i) * 8] = true;
            }

            if hor as i8 - i as i8 >= 0 && ver as i8 - i as i8 >= 0 {
                board[hor - i + (ver - i) * 8] = true;
            }
        }
    }

    true
}