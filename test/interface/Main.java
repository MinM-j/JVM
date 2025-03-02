// Main class to test the interface and its implementations
public class Main {
    public static void main(String[] args) {
        // Create a Rectangle object
        Rectangle rectangle = new Rectangle(5.0, 3.0);
		double rect_peri = rectangle.perimeter();
		double rect_area = rectangle.area();

        // Create a Triangle object
		Triangle triangle = new Triangle(3.0, 4.0, 5.0, 4.0, 3.0);
		double tri_peri = triangle.perimeter();
		double tri_area = triangle.area();
    }
}
