// Homework for 31/8/2023
// Check if a number is divisible by 5 or 7

import java.util.Scanner;

public class div_by_5_or_7 {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Enter a number: ");
        int num = input.nextInt();

        boolean divisible = num % 5 == 0 || num % 7 == 0;

        System.out.println(num + " is divisible by 5 or 7: " + divisible);

        input.close();
    }
}
