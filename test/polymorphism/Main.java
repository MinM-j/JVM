

public class Main {
    public static void main(String[] args) {
        // Creating objects of Dog and Cat using the Animal interface
        Animal myDog = new Dog();
        Animal myCat = new Cat();

        // Demonstrating runtime polymorphism (method overriding)
        String dog_sound = myDog.sound();  // Calls Dog's sound() method
        String cat_sound = myCat.sound();  // Calls Cat's sound() method

        // Demonstrating method overloading (compile-time polymorphism)
        String dog_move = myDog.move(); // Calls the Dog's move() method
        String cat_move = myCat.move();
        //myDog.move("jumping"); // Calls the Dog's move() method with parameter
    }
}

