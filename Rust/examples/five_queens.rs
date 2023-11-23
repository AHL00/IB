// 5 queens on an 8x8 board covering every square

fn main() {
    check(&[28]);

    let mut solutions = vec![];

    for i in 0..64 {
        for j in i + 1..64 {
            for k in j + 1..64 {
                for l in k + 1..64 {
                    for m in l + 1..64 {
                        let queens = [i, j, k, l, m];

                        if check(&queens) {
                            solutions.push(queens);
                        }
                    }
                }
            }
        }
    }

    println!("Done");
    println!("Solutions: {:?}", solutions[0]);

    print_board_from_queens(&solutions[0]);
    

}

fn print_board_from_queens(queens: &[u8]) {
    let mut board = [false; 64];

    for queen in queens.iter() {
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

    for i in 0..64 {
        print!(" ");
        
        if queens.contains(&(i as u8)) {
            print!("Q");
        }
        else if board[i] {
            print!("X");
        } else {
            print!(".");
        }

        if i % 8 == 7 {
            println!();
        }
    }

    println!();
}

fn check(queens: &[u8]) -> bool {
    let mut board = [false; 64];

    for queen in queens.iter() {
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

    for i in 0..64 {
        if !board[i] {
            return false;
        }
    }

    true
}