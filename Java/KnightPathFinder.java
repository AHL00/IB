public class KnightPathFinder {
    int size_x;
    int size_y;

    int[][] board;

    int start_x;
    int start_y;

    public static void main(String[] args) {
        KnightPathFinder k = new KnightPathFinder(0, 0, 100, 10);

        boolean result = k.can_fill_board();

        System.out.println("Result: " + result);

    }

    public KnightPathFinder(int start_x, int start_y, int size_x, int size_y) {
        this.size_x = size_x;
        this.size_y = size_y;
        this.start_x = start_x;
        this.start_y = start_y;
        this.board = new int[size_x][size_y];
    }

    public boolean can_fill_board() {
        board[start_x][start_y] = 1;

        int i = 1;

        while (true) {
            // Check if the board is filled
            // If so, break
            boolean is_filled = true;
            for (int x = 0; x < size_x; x++) {
                for (int y = 0; y < size_y; y++) {
                    if (board[x][y] == 0) {
                        is_filled = false;
                        break;
                    }
                }
            }

            if (is_filled) {
                return true;
            }

            int moves_made = 0;

            // Find all of the current moves
            // For each current move, fill the possibilities with the next move number
            for (int x = 0; x < size_x; x++) {
                for (int y = 0; y < size_y; y++) {
                    if (board[x][y] == i) {
                        boolean moved = fill_possible(x, y, i + 1);

                        if (moved) {
                            moves_made++;
                        }
                    }
                }
            }

            // If not moves were made, that means
            // the board cannot be filled
            if (moves_made == 0) {
                return false;
            }

            System.out.println("Moves made at move index " + i + ": " + moves_made);

            // Next move
            i++;
        }
    }

    /// Returns whether at least one move is possible
    boolean fill_possible(int x, int y, int move_num) {
        int[][] possible_moves = {
                { x + 2, y + 1 },
                { x + 2, y - 1 },
                { x - 2, y + 1 },
                { x - 2, y - 1 },
                { x + 1, y + 2 },
                { x + 1, y - 2 },
                { x - 1, y + 2 },
                { x - 1, y - 2 }
        };

        boolean moved = false;

        // If not negative or out of bounds or already filled
        // Set the board to the move number
        for (int[] move : possible_moves) {
            if (move[0] >= 0 && move[0] < size_x && move[1] >= 0 && move[1] < size_y && board[move[0]][move[1]] == 0) {
                board[move[0]][move[1]] = move_num;
                moved = true;
            }
        }

        return moved;
    }
}