// Base class
class Animal {
    // Instance variables
    String name;
    int age;

    // Constructor for Animal
    public Animal(String name, int age) {
        this.name = name;
        this.age = age;
    }

    // Method to display animal information
    public void speak() {
         ioTer.prints("Animal is making a sound");
    }

    // Method to display general information about the animal
    public void displayInfo() {
		ioTer.prints(name);
		ioTer.prints("Age:");
		ioTer.printn(age);
    }
}
