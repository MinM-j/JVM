class Triangle implements Shape {
    double side1;
    double side2;
    double side3;
    double base;
    double height;

    public Triangle(double side1, double side2, double side3, double base, double height) {
        this.side1 = side1;
        this.side2 = side2;
        this.side3 = side3;
        this.base = base;
        this.height = height;
    }

    @Override
    public double perimeter() {
        return side1 + side2 + side3;
    }

    @Override
    public double area() {
        return 0.5 * base * height;
    }
}
