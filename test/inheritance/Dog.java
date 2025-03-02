// Subclass Dog inheriting from Animal
class Dog extends Animal {
    // Constructor for Dog
    public Dog(String name, int age) {
        super(name, age); // Calls the constructor of the parent class (Animal)
    }

    // Overriding the speak method
    @Override
    public String speak() {
        return "The dog barks";
    }
}

