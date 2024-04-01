import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Main {

    public static void main(String[] args) {
        int[] test_grid_size = { 4, 4 };
        Boolean[] test_grid = {
                true, true, false, false,
                false, true, false, false,
                true, false, false, true,
                false, false, false, true
        };

        int island_count = find_islands(new BitArray(Arrays.asList(test_grid)), test_grid_size);

        System.out.println("Island count: " + island_count);

        int[] test_grid_2_size = { 10, 10 };

        Boolean[] test_grid_2 =

                {
                    true, true, false, false, false, false, false, false, true, false,
                    true, true, false, false, false, false, false, false, false, true,
                    false, true, false, false, false, false, false, false, true, false,
                    true, false, false, true, false, false, false, false, false, false,
                    false, false, true, false, false, false, false, false, false, false,
                    false, false, false, false, true, true, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false,
                    false, false, false, true, false, false, false, false, true, true,
                    false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false
                };

                // Checkerboard pattern
                // {
                //     true, false, true, false, true, false, true, false, true, false,
                //     false, true, false, true, false, true, false, true, false, true,
                //     true, false, true, false, true, false, true, false, true, false,
                //     false, true, false, true, false, true, false, true, false, true,
                //     true, false, true, false, true, false, true, false, true, false,
                //     false, true, false, true, false, true, false, true, false, true,
                //     true, false, true, false, true, false, true, false, true, false,
                //     false, true, false, true, false, true, false, true, false, true,
                //     true, false, true, false, true, false, true, false, true, false,
                //     false, true, false, true, false, true, false, true, false, true
                // };

        // Pretty print the above array
        for (int i = 0; i < test_grid_2.length; i++) {
            if (i % test_grid_2_size[0] == 0) {
                System.out.println();
            }

            if (test_grid_2[i] == true) {
                System.out.print("██");
            } else {
                System.out.print("░░");
            }
        }
        System.out.println();

        System.out.println("Island count: " + find_islands(new BitArray(Arrays.asList(test_grid_2)), test_grid_2_size));

        // Random grid
        int[] random_grid_size = { 100000, 100000 };

        BitArray random_grid = new BitArray(random_grid_size[0] * random_grid_size[1]);

        for (int i = 0; i < random_grid_size[0] * random_grid_size[1]; i++) {
            random_grid.set(i, Math.random() < 0.3);
        }

        System.out.println("Random grid generated");
        System.out.println("Array size: " + random_grid.size());

        // Pretty print the above array
        // for (int i = 0; i < random_grid.length; i++) {
        // if (i % random_grid_size[0] == 0) {
        // System.out.println();
        // }

        // if (random_grid[i] == true) {
        // System.out.print("█");
        // } else {
        // System.out.print("░");
        // }
        // }
        // System.out.println();

        System.out.println("Island count: " + find_islands(random_grid, random_grid_size));
    }

    static int find_islands(BitArray grid, int[] grid_size) {
        int count = 0;

        // Iterate over array, if land found where checked is not true,
        // then it is a new island.
        // In this case, call another function to process the entire island, maybe
        // recursive.
        for (int i = 0; i < grid.size(); i++) {
            if (grid.get(i) == true) {
                // New island found
                consume_island(grid, i, grid_size);

                count++;
            }
        }

        return count;
    }

    /// Recursive function to check if neighbors are land and set them as checked
    /// Grid size is {x, y}
    static void consume_island(BitArray grid, int location, int[] grid_size) {
        // Make self 0
        grid.set(location, false);

        // Check neighbors
        ArrayList<int[]> neighbor_offsets = new ArrayList<int[]>();

        // System.out.println("Location: " + location);

        boolean is_top = location < grid_size[0];
        boolean is_bottom = (grid.size() - grid_size[0]) < location;
        boolean is_left = (location % grid_size[0]) == 0;
        boolean is_right = (location % grid_size[0]) == (grid_size[0] - 1);

        // System.out.println("Is top: " + is_top);
        // System.out.println("Is bottom: " + is_bottom);
        // System.out.println("Is left: " + is_left);
        // System.out.println("Is right: " + is_right);

        if (!is_top) {
            // If not at top, check block above
            int[] offset = { 0, -1 };
            neighbor_offsets.add(offset);
        }

        if (!is_bottom) {
            // If not at bottom, check block below
            int[] offset = { 0, 1 };
            neighbor_offsets.add(offset);
        }

        if (!is_left) {
            // If not at left, check block to the left
            int[] offset = { -1, 0 };
            neighbor_offsets.add(offset);
        }

        if (!is_right) {
            // If not at right, check block to the right
            int[] offset = { 1, 0 };
            neighbor_offsets.add(offset);
        }

        if (!is_left && !is_top) {
            // If not at top or left, check block above and to the left
            int[] offset = { -1, -1 };
            neighbor_offsets.add(offset);
        }

        if (!is_right && !is_bottom) {
            // If not at bottom or right, check block below and to the right
            int[] offset = { 1, 1 };
            neighbor_offsets.add(offset);
        }

        if (!is_right && !is_top) {
            // If not at top or right, check block above and to the right
            int[] offset = { 1, -1 };
            neighbor_offsets.add(offset);
        }

        if (!is_left && !is_bottom) {
            // If not at bottom or left, check block below and to the left
            int[] offset = { -1, 1 };
            neighbor_offsets.add(offset);
        }

        // System.out.println("Neighbor offsets: ");

        // for (int[] offset : neighbor_offsets) {
        // System.out.println(Arrays.toString(offset));
        // }

        // System.out.println("------------------------");

        for (int[] offset : neighbor_offsets) {
            int neighbor_location = location + offset[0] + offset[1] * grid_size[0];

            // Hardcode bug fix
            if (!(neighbor_location >= grid.size())) {
                if (grid.get(neighbor_location) == true) {

                    consume_island(grid, neighbor_location, grid_size);
                }
            }
        }
    }
}

class BitArray {
    private int[] data;
    private int size;

    public BitArray(int size) {
        this.size = size;
        this.data = new int[(size + 31) / 32];
    }

    public BitArray(List<Boolean> bits) {
        this(bits.size());
        for (int i = 0; i < bits.size(); i++) {
            set(i, bits.get(i));
        }
    }

    public void set(int index, boolean value) {
        if (value) {
            data[index / 32] |= 1 << (index % 32);
        } else {
            data[index / 32] &= ~(1 << (index % 32));
        }
    }

    public boolean get(int index) {
        return (data[index / 32] & (1 << (index % 32))) != 0;
    }

    public int size() {
        return size;
    }
}