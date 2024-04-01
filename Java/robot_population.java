import java.util.LinkedList;

public class robot_population {
    /// The values are lifetimes
    LinkedList<Integer> robots = new LinkedList<Integer>();

    public static void main(String[] args) {
        robot_population r = new robot_population(10);

        int max = r.calc_max(5);

        System.out.println("Max: " + max);
    }

    void add_new_robots(int count) {
        for (int i = 0; i < count; i++) {
            robots.add(3);
        }
    }

    robot_population(int robot_count) {
        if (robot_count < 3) {
            System.out.println("Robot count must be at least 3");
            System.exit(1);
        }

        add_new_robots(robot_count);
    }

    int calc_max(int years) {
        int max = 0;

        for (int i = 0; i < years; i++) {
            // 3 robots -> 5 produced
            // 5 robots -> 9 produced
            LinkedList<Integer> production_groups = new LinkedList<Integer>();
            
            // Calculate production schedule
            // Try and maximise numbers of 5's
            int fives = robots.size() / 5;
            int fives_remainder = robots.size() % 5;

            for (int j = 0; j < fives; j++) {
                production_groups.add(5);
            }

            switch (fives_remainder) {
                case 1:
                    production_groups.pop();
                    production_groups.add(3);
                    production_groups.add(3);

                    break;
                case 2:
                    if (robots.size() >= 12) {
                        // Remove two 5's and add 4 3's
                        production_groups.pop();
                        production_groups.pop();

                        production_groups.add(3);
                        production_groups.add(3);
                        production_groups.add(3);
                        production_groups.add(3);
                    } 
                    else {
                        // This case is 100% sure to be 7 robots
                        // Remove one 5 and add 2 3's
                        production_groups.pop();
                        production_groups.add(3);
                        production_groups.add(3);
                    }         

                    break;
                case 3:
                    production_groups.add(3);

                    break;
                case 4:
                    production_groups.pop();
                    production_groups.add(3);
                    production_groups.add(3);
                    production_groups.add(3);

                    break;
            }

            // Print prod array
            // System.out.println("Production groups: " + Arrays.toString(production_groups.toArray()));

            // Add produced robots
            for (int group: production_groups) {
                if (group == 3) {
                    add_new_robots(5);
                } else if (group == 5) {
                    add_new_robots(9);
                }
            }

            // Update lifetimes
            // System.out.println("Lifetimes: " + Arrays.toString(robots.toArray()));
            for (int j = robots.size() - 1; j >= 0; j--) {

                
                // If lifetime is 0, remove
                if (robots.get(j) == 0) {
                    robots.remove(j);
                    System.out.println("Robot removed");
                }

                robots.set(j, robots.get(j) - 1);
            }

            System.out.println("Year " + (i + 1) + " robots: " + robots.size());

            if (robots.size() > max) {
                max = robots.size();
            }
        }

        return max;
    }

}