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
    public String speak() {
        return "Animal is making a sound";
    }

    // Method to display general information about the animal
    public String displayInfo() {
		return name;
    }
}
