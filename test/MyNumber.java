public class MyNumber {
    private int x;
    private int y;

    // Constructor
    public MyNumber(int x, int y) {
        this.x = x;
        this.y = y;
    }

    // Add method
    public MyNumber add(MyNumber other) {
        int newX = this.x + other.x;
        int newY = this.y + other.y;
        return new MyNumber(newX, newY);
    }

    public void display() {
        ioTer.printi(x);
        ioTer.printi(y);
    }

    // Main method to test
    public static void main(String[] args) {
        MyNumber num1 = new MyNumber(10, 20);
        MyNumber num2 = new MyNumber(5, 15);

        MyNumber result = num1.add(num2);
        result.display();

    }
}
