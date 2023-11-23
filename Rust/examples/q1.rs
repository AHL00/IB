use painless_input;

fn main() {
    let mut den: i32 = painless_input::input("Input denary number: ");
    println!();

    let mut multi = 1;

    let mut bin = 0;

    while den > 0 {
        bin += (den % 2) * multi;

        den /= 2;
        multi *= 10;
    }

    println!("In binary: {}", bin);
}

// import java.util.Scanner;
//
// class Main {
// public static void main(String[] args) {
// Scanner sc = new Scanner(System.in);
//
// System.out.println("Enter a number: ");
//
// int num = sc.nextInt();
//
// int multi = 1;
// int bin = 0;
//
// while (num > 0) {
// bin += (num % 2) * multi;
//
// num /= 2;
// multi *= 10;
// }
//
// System.out.println(bin);
// }
// }