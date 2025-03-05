// Main class to test the interface and its implementations
public class Main {
    public static void main(String[] args) {
        // Create a Rectangle object
        Rectangle rectangle = new Rectangle(5.0, 3.0);
		ioTer.prints("Rectangle perimeter:");
		ioTer.printn(rectangle.perimeter());
		ioTer.prints("Rectangle area:");
		ioTer.printn(rectangle.area());

        // Create a Triangle object
		Triangle triangle = new Triangle(3.0, 4.0, 5.0, 4.0, 3.0);
		ioTer.prints("Triangle perimeter:");
		ioTer.printn(triangle.perimeter());
		ioTer.prints("Triangle area:");
		ioTer.printn(triangle.area());
    }
}
