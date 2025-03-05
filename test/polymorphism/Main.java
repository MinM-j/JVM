

public class Main {
    public static void main(String[] args) {
        // Creating objects of Dog and Cat using the Animal interface
        Animal myDog = new Dog();
        Animal myCat = new Cat();

        // Demonstrating runtime polymorphism (method overriding)
        myDog.sound();  // Calls Dog's sound() method
        myCat.sound();  // Calls Cat's sound() method

        // Demonstrating method overloading (compile-time polymorphism)
        myDog.move(); // Calls the Dog's move() method
        myCat.move();
        //myDog.move("jumping"); // Calls the Dog's move() method with parameter
    }
}

