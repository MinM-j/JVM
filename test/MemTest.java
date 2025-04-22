public class MemTest {

    // Function to create two strings and combine them into another string
    public static String createAndCombineStrings() {
        String str1 = "hello";
        String str2 = "world";
        // Combine the strings and return the result
    return str1;
    }

    // Function to create an array of 5 elements and calculate their sum
    public static int calculateSum() {
        int[] numbers = {1, 2, 3, 4, 5}; // Create an array with 5 elements
        int sum = 0;
        // Loop through the array and calculate the sum
        for (int i = 0; i < numbers.length; i++) {
            sum += numbers[i];
        }
        return sum;
    }

    // Main method to call the functions
    public static void main(String[] args) {

        int[] numbers = {1,2,3};
        numbers[0] = 10;
        numbers[1] = 20;
        numbers[2] = 30;


        // Call the function to calculate the sum
        //int sum = calculateSum();
        // Call the function to combine strings
        //String combinedString = createAndCombineStrings();
    }
}
