public class digit_wordle {
    public static void main(String[] args) {
        int digits = 4;

        // Ask how many digits the user wants to play
        System.out.println("How many digits do you want to play with? ");
        digits = Integer.parseInt(System.console().readLine());

        int ten_to_digits = (int) Math.pow(10, digits - 1);

        int n = ten_to_digits + (int) (Math.random() * 9 * ten_to_digits);

        // Clear terminal
        System.out.print("\033[H\033[2J");

        // Reset cursor
        System.out.print("\033[0;0H");

        int guess_count = 0;

        // Main loop
        while (true) {
            // Ask user to guess the number
            int guess = -1;
            
            System.out.print("Guess: ");
            while (guess == -1) {
                try {
                    int input = Integer.parseInt(System.console().readLine().split(" ")[0]);

                    // Delete line
                    System.out.print("\033[1A\033[2K");

                    if (input < ten_to_digits || input >= 10 * ten_to_digits) {
                        System.out.print("Invalid input, guess again: ");
                    } else {
                        guess = input;
                    }
                } catch (NumberFormatException e) {
                    System.out.print("Invalid input, guess again: ");

                    continue;
                }
            }

            guess_count++;

            // Check each digit and check if it's in the number
            int[] guess_digits = new int[digits];
            int[] n_digits = new int[digits];

            for (int i = 0; i < digits; i++) {
                guess_digits[i] = guess / (int) Math.pow(10, digits - i - 1) % 10;
                n_digits[i] = n / (int) Math.pow(10, digits - i - 1) % 10;
            }

            int correct_count = 0;

            for (int i = 0; i < digits; i++) {
                // Check if the digit is in the correct position
                if (guess_digits[i] == n_digits[i]) {
                    /// Print the number with a green background
                    System.out.print("\033[42m" + n_digits[i] + "\033[49m ");

                    correct_count++;

                    continue;
                }

                boolean in_number = false;
                for (int j = 0; j < digits; j++) {
                    // Check if the digit is in the number
                    if (guess_digits[i] == n_digits[j]) {
                        in_number = true;
                    }
                }

                if (!in_number) {
                    // Print the number with a red background
                    System.out.print("\033[41m" + guess_digits[i] + "\033[49m ");
                } else {
                    // Print the number with a yellow background
                    System.out.print("\033[43m" + guess_digits[i] + "\033[49m ");
                }
            }

            if (correct_count == digits) {
                System.out.println("\nYou won!");
                
                // Summary
                System.out.println("Number: " + n);
                System.out.println("Guesses: " + guess_count);

                break;
            }

            // Next line
            System.out.println();
        }
    }
}