

public class perfect_numbers {
    public static void main(String[] args) {
        int n = 3;
        int n2 = 1;

        while (true) {
            long x = ((long) 2 << (n - 1)) - ((long) 2 << (n2 - 1));

            if (x <= 0) {
                break;
            }
            
            System.out.println("Checking " + x);

            long sum = 0;

            long sqrt_x = (long) Math.floor(Math.sqrt((double) x));

            for (long i = 2; i <= sqrt_x; i++) {
                if (x % i == 0) {
                    sum += i;
                    sum += x / i;
                }
            }    
            
            sum += 1;

            if (x == sum) {
                System.out.println("Perfect num: " + x);
            }

            // Calc next N
            n += 2;

            n2 = (n - 1) / 2;
        }
    }
}
