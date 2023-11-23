import java.util.Scanner;

public class sentence_contains_palindrome {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Sentence: ");

        String sentence = input.nextLine();

        boolean contains_palindrome = false;

        // Iterate over words
        // Nested for then while
        for (int i = 0; i < sentence.length(); i++) {
            // Check if word is palindrome
            int j = i;
            int k = i;

            while (j >= 0 && k < sentence.length() && sentence.charAt(j) == sentence.charAt(k)) {
                j--;
                k++;
            }

            if (k - j > 2) {
                contains_palindrome = true;
                break;
            }
        }
        

        input.close();
    }
}