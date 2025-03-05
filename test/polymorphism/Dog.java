
// Dog class implements the Animal interface
class Dog implements Animal {
    // Implementing the sound method (runtime polymorphism)
    @Override
    public void sound() {
        ioTer.prints("Dog barks");
    }

    // Implementing the move method (overloading and runtime polymorphism)
    @Override
    public void move() {
        ioTer.prints("Dog is walking");
    }

}

