import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;

/// Created an algorithm called the "cascade down" algorithm
/// Fast for smaller ratios of k to n

public class KthLargest {

    public enum CascadeMode {
        LINEAR, BINARY
    }

    public static CascadeMode mode = CascadeMode.LINEAR;

    public static void main(String[] args) throws IOException {
        String dataStr = new String(Files.readAllBytes(Paths.get("../5000_random.txt")));

        int[] arr = new int[5000];
        String[] lines = dataStr.split("\n");

        for (int i = 0; i < lines.length; i++) {
            arr[i] = Integer.parseInt(lines[i].replace(",", ""));
        }

        // int[] arr = new int[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 10, 10, 10, 5, 6, 7 };

        int k = 4000;

        int[] topKVec = new int[k];
        Arrays.fill(topKVec, -1);

        /* Start timing */
        long start = System.nanoTime();

        for (int n : arr) {
            cascadeDown(topKVec, 0, n);
        }

        long elapsed = System.nanoTime() - start;

        System.out.println("Top " + k + " element: " + topKVec[k - 1]);
        System.out.println("Time taken: " + elapsed + " ns");
        System.out.println("Time taken: " + elapsed / 1000000 + " ms");
        // System.out.println("Array: " + Arrays.toString(topKVec));
    }

    /// "Cascade" this number down, swapping until we find a -1 or a number that is
    /// smaller
    /// If we get to the end, that means this number is the smallest of the top k
    /// and gets replaced
    private static void cascadeDown(int[] arr, int startIdx, int n) {
        if (mode == CascadeMode.LINEAR) {
            int i = startIdx;

            // This is basically a linear search
            while (i < arr.length) {
                // If the element is -1, replace it
                if (arr[i] == -1) {
                    arr[i] = n;
                    break;
                }

                // If the element is smaller than n, replace it and cascade it down
                if (arr[i] < n) {
                    int replaced = arr[i];
                    arr[i] = n;
                    n = replaced;
                }

                i++;
            }
        } else {
            // Optimised version of the above loop using a binary search
            // arr is in descending order
            int left = startIdx;
            int right = arr.length;

            while (left < right) {
                int mid = (left + right) / 2;

                if (arr[mid] < n) {
                    /// If we are at the start of the array, replace the first element
                    if (mid == 0 || n <= arr[mid - 1]) {
                        int replaced = arr[mid];

                        // This means that n is between mid and mid + 1
                        // That means we replace the bigger (mid) with n
                        arr[mid] = n;

                        // Cascade replaced down
                        // cascadeDown(arr, mid, replaced);
                        // break;

                        n = replaced;
                        left = mid;
                        right = arr.length;
                        continue;
                    }

                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }
    }
}