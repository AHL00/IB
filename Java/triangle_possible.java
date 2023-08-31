// Homework for 31/8/2023
// Triangle possible given three sides?

import java.util.Scanner;

public class triangle_possible {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Side a: ");
        int a = input.nextInt();

        System.out.print("Side b: ");
        int b = input.nextInt();

        System.out.print("Side c: ");
        int c = input.nextInt();

        boolean possible = a + b > c && a + c > b && b + c > a;

        System.out.println("Triangle possible: " + possible);

        input.close();
    }
}