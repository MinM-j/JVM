// Subclass Cat inheriting from Animal
class Cat extends Animal {
    // Constructor for Cat
    public Cat(String name, int age) {
        super(name, age); // Calls the constructor of the parent class (Animal)
    }

    // Overriding the speak method
    @Override
    public void speak() {
        ioTer.prints("The cat meows");
    }
}
