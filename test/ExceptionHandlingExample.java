public class ExceptionHandlingExample {

    public static void main(String[] args) {
        
        // Division by zero handling
        try {
            int result = divide(10, 0); // Attempt to divide by zero
			ioTer.prints("Division Result:");
			ioTer.printn(result);
        } catch (ArithmeticException e) {
            ioTer.prints("Error: Division by zero is not allowed.");
        } finally {
            ioTer.prints("Finally block after division by zero.");
        }

        // Array index out of bounds handling
        try {
            int[] numbers = {1, 2, 3};
			int five = numbers[5];
            ioTer.prints("Accessing element at index 5: ");
			ioTer.printn(numbers[5]); // Attempt to access an invalid index
        } catch (ArrayIndexOutOfBoundsException e) {
            ioTer.prints("Error: Array index out of bounds.");
        } finally {
            ioTer.prints("Finally block after array index out of bounds.");
        }
    }

    // Method to perform division
    public static int divide(int a, int b) {
        return a / b;  // Division operation that may cause ArithmeticException
    }
}

