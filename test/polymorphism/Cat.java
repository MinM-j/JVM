// Cat class implements the Animal interface
class Cat implements Animal {
    // Implementing the sound method (runtime polymorphism)
    @Override
    public void sound() {
        ioTer.prints("Cat meows");
    }

    // Implementing the move method (overloading and runtime polymorphism)
    @Override
    public void move() {
        ioTer.prints("Cat is walking");
    }
}
