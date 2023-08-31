// Homework for 31/8/2023
// Given the coords of two points on a chess board, a knight can perform the move

import java.util.Scanner;

public class knight_chess_possible {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Enter the first x coord: ");
        int x1 = input.nextInt();

        System.out.print("Enter the first y coord: ");
        int y1 = input.nextInt();

        System.out.print("Enter the second x coord: ");
        int x2 = input.nextInt();

        System.out.print("Enter the second y coord: ");
        int y2 = input.nextInt();

        int dx = Math.abs(x1 - x2);
        int dy = Math.abs(y1 - y2);

        boolean possible = (dx == 1 && dy == 2) || (dx == 2 && dy == 1);

        System.out.println("The move is possible: " + possible);

        input.close();
    }
}
