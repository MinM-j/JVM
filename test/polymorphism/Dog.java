
// Dog class implements the Animal interface
class Dog implements Animal {
    // Implementing the sound method (runtime polymorphism)
    @Override
    public String sound() {
        return "Dog barks";
    }

    // Implementing the move method (overloading and runtime polymorphism)
    @Override
    public String move() {
        return "Dog is walking";
    }

    // Overloaded move method with additional parameter
 //   @Override
 //   public String move(String mode) {
 //       System.out.println("Dog is moving by " + mode);
 //   }
}

