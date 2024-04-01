import java.util.Arrays;
import java.util.LinkedList;
import java.util.Vector;

public class balance {
    public static void main(String[] args) {
        LinkedList<Integer> weight_list = new LinkedList<Integer>(Arrays.asList(1, 2, 5, 10, 20, 50, 100, 200, 500));

        int weight = 0;

        /// The side with the item on it
        LinkedList<Integer> left = new LinkedList<Integer>();
        /// The side without the item on it
        LinkedList<Integer> right = new LinkedList<Integer>();

        // Input weight
        while (true) {
            System.out.print("Weight: ");
            try {
                int input = Integer.parseInt(System.console().readLine().split(" ")[0]);

                if (input < 1 || input > 888) {
                    System.out.println("Invalid input, try again");
                } else {
                    weight = input;
                    break;
                }
            } catch (NumberFormatException e) {
                System.out.println("Invalid input, try again");
            }
        }

        calc(weight_list, weight, left, right, false);

        System.out.println("The weight is on the left side");
        System.out.println("Left: " + left);
        System.out.println("Right: " + right);
    }

    static void calc(LinkedList<Integer> weight_list, int weight, LinkedList<Integer> left, LinkedList<Integer> right,
            boolean is_simulated) {
        // If the weight is 0, we're done
        if (weight == 0) {
            return;
        }

        // If weight list is empty, we're done
        if (weight_list.size() == 0) {
            return;
        }

        // If the weight is bigger than the next weight, we can add it to the right side
        if (weight >= weight_list.get(weight_list.size() - 1)) {
            right.add(weight_list.get(weight_list.size() - 1));

            // Calc new weight
            weight -= weight_list.get(weight_list.size() - 1);

            // Remove the weight from the list
            weight_list.remove(weight_list.size() - 1);

            // Recurse
            calc(weight_list, weight, left, right, is_simulated);

            return;
        } else {
            // If the weight is smaller than the next weight
            if (weight == -1)
                System.out.println("Current number: " + weight);

            // Check if the differences can form a weight than can be calculated
            for (int i = 0; i < weight_list.size(); i++) {
                for (int j = 0; j < weight_list.size(); j++) {
                    if (i == j) {
                        continue;
                    }

                    // If placing i on the left and j on the right
                    int new_weight = weight + weight_list.get(i) - weight_list.get(j);

                    if ((weight == -1) || !is_simulated)
                        System.out.println("Trying left [" + weight_list.get(i) + "] and right [" + weight_list.get(j)
                                + "] = " + new_weight);
                    else if (weight == -1)
                        System.out.println("Simulated: Trying left [" + weight_list.get(i) + "] and right ["
                                + weight_list.get(j) + "] = " + new_weight);

                    // If new weight is 0, we're done
                    // If new weight can be formed with the remaining weights
                    if (new_weight == 0) {
                        left.add(weight_list.get(i));
                        right.add(weight_list.get(j));

                        // Remove the weights from the list
                        if (i > j) {
                            weight_list.remove(i);
                            weight_list.remove(j);
                        } else {
                            weight_list.remove(j);
                            weight_list.remove(i);
                        }

                        return;
                    } else {
                        // Try running on a simulated list
                        LinkedList<Integer> sim_weight_list = new LinkedList<Integer>(weight_list);

                        if (i > j) {
                            sim_weight_list.remove(i);
                            sim_weight_list.remove(j);
                        } else {
                            sim_weight_list.remove(j);
                            sim_weight_list.remove(i);
                        }

                        LinkedList<Integer> sim_left = new LinkedList<Integer>(left);
                        LinkedList<Integer> sim_right = new LinkedList<Integer>(right);

                        int sim_weight = new_weight;

                        calc(sim_weight_list, sim_weight, sim_left, sim_right, true);
                        
                        if (weight == -1)
                            System.out.println("Sim weight: " + sim_weight);

                            
                        if (sim_weight == 0) {
                            // Then we know this method will work in the future, so this works
                            System.out.println("SOLUTION FOUND");
                            System.out
                                    .println("Adding left [" + weight_list.get(i) + "] and right [" + weight_list.get(j)
                                            + "]");
                            left.add(weight_list.get(i));
                            right.add(weight_list.get(j));

                            // Remove the weights from the list
                            if (i > j) {
                                sim_weight_list.remove(i);
                                sim_weight_list.remove(j);
                            } else {
                                sim_weight_list.remove(j);
                                sim_weight_list.remove(i);
                            }

                            return;
                        }
                    }
                }
            }

            if (!is_simulated)
                System.out.println("No solution, skipping " + weight_list.get(weight_list.size() - 1));

            // Skip the next weight
            weight_list.remove(weight_list.size() - 1);

            // Recurse
            calc(weight_list, weight, left, right, is_simulated);
        }

    }
}