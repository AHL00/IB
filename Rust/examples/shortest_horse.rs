const MOVES: [(i32, i32); 8] = [(2, 1), (2, -1), (-2, 1), (-2, -1),(1, 2), (1, -2), (-1, 2), (-1, -2)];
const SIZE: usize = 8;

fn main() {
    let a_vec = painless_input::input_array("Enter the start coords: ");
    println!();
    let b_vec = painless_input::input_array("Enter the end coords: ");
    println!();

    let start_time = std::time::Instant::now();

    let start: (usize, usize) = (a_vec[0], a_vec[1]);
    let end: (usize, usize) = (b_vec[0], b_vec[1]);

    let mut board: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut current_num = 2;

    board[start.0][start.1] = 1;
    board[end.0][end.1] = -1;

    // Fill board
    'outer: loop {
        for x in 0..SIZE {
            for y in 0..SIZE {
                if board[x][y] == current_num - 1 {
                    // Set surrounding moves to current_num
                    for k in 0..8 {
                        let (new_x, new_y) = (x as i32 + MOVES[k].0, y as i32 + MOVES[k].1);

                        if new_x >= 0 && new_x < SIZE as i32 && new_y >= 0 && new_y < SIZE as i32 {
                            if board[new_x as usize][new_y as usize] == 0 {
                                board[new_x as usize][new_y as usize] = current_num;
                            }

                            if board[new_x as usize][new_y as usize] == -1 {
                                board[new_x as usize][new_y as usize] = current_num;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        current_num += 1;
    }

    let mut steps = std::collections::VecDeque::new();
    let mut current_pos = end;
    while current_num > 1 {
        // Find current num - 1 in possible moves
        for mv in MOVES.iter() {
            let (new_x, new_y) = (current_pos.0 as i32 + mv.0, current_pos.1 as i32 + mv.1);

            if new_x >= 0 && new_x < SIZE as i32 && new_y >= 0 && new_y < SIZE as i32 {
                if board[new_x as usize][new_y as usize] == current_num - 1 {
                    // Add the move to the steps
                    steps.push_front((-mv.0, -mv.1));
                    current_pos = (new_x as usize, new_y as usize);
                }
            }
        }

        current_num -= 1;
    }
    
    println!("Time taken: {:?}\nSteps: {:?}", start_time.elapsed(), steps);
}