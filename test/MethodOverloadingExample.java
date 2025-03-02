public class MethodOverloadingExample {

    // Method to sum two integers
    public int sum(int a, int b) {
        return a + b;
    }

    // Overloaded method to sum three integers
    public int sum(int a, int b, int c) {
        return a + b + c;
    }

    // Overloaded method to sum two doubles
    public double sum(double a, double b) {
        return a + b;
    }

    public static void main(String[] args) {
        MethodOverloadingExample example = new MethodOverloadingExample();

        int sum1 = example.sum(5, 10); // Outputs 15

        int sum2 = example.sum(5, 10, 15); // Outputs 30

        double sum3 = example.sum(5.5, 10.5); // Outputs 16.0
    }
}

