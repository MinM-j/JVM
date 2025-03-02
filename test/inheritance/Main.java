// Main class to test inheritance
public class Main {
    public static void main(String[] args) {
        // Creating objects of Dog and Cat
        Dog dog = new Dog("Buddy", 3);
        Cat cat = new Cat("Whiskers", 2);

		String dog_info = dog.displayInfo();
		String dog_speak = dog.speak();

		String cat_info = cat.displayInfo();
		String cat_speak = cat.speak();
    }
}

