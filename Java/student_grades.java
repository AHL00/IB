import java.util.Scanner;

public class student_grades {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Enter the score: ");
        int score = input.nextInt();

        String grade = "D";

        if (score >= 75) {
            grade = "A";
        } else if (score >= 50) {
            grade = "B";
        } else if (score >= 25) {
            grade = "C";
        }

        System.out.println("The grade is: " + grade);

        input.close();
    }
}
