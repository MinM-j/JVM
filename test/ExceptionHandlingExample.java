public class ExceptionHandlingExample {

    public static void main(String[] args) {
        
        // Division by zero handling
        try {
            int result = divide(10, 1); // Attempt to divide by zero
            //System.out.println("Division Result: " + result);
        } catch (ArithmeticException e) {
			int super_damn = 1;
            //System.out.println("Error: Division by zero is not allowed.");
        } finally {
			double super_damn = 3.0;
            //System.out.println("Finally block after division by zero.");
        }

        // Array index out of bounds handling
        try {
            int[] numbers = {1, 2, 3};
			int five = numbers[5];
            //System.out.println("Accessing element at index 5: " + numbers[5]); // Attempt to access an invalid index
        } catch (ArrayIndexOutOfBoundsException e) {
			long super_damn_1 = 1;
            //System.out.println("Error: Array index out of bounds.");
        } finally {
			long super_damn_1 = 3;
            //System.out.println("Finally block after array index out of bounds.");
        }
    }

    // Method to perform division
    public static int divide(int a, int b) {
        return a / b;  // Division operation that may cause ArithmeticException
    }
}

