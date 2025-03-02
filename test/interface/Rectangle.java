// Rectangle class implements Shape interface
class Rectangle implements Shape {
    double length;
    double width;

    // Constructor for Rectangle
    public Rectangle(double length, double width) {
        this.length = length;
        this.width = width;
    }

    // Implement the perimeter method for Rectangle
    @Override
    public double perimeter() {
        return 2 * (length + width);
    }

    // Implement the area method for Rectangle
    @Override
    public double area() {
        return length * width;
    }
}
