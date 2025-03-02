// Cat class implements the Animal interface
class Cat implements Animal {
    // Implementing the sound method (runtime polymorphism)
    @Override
    public String sound() {
        return "Cat meows";
    }

    // Implementing the move method (overloading and runtime polymorphism)
    @Override
    public String move() {
        return "Cat is walking";
    }

    // Overloaded move method with additional parameter
 //   @Override
 //   public String move(String mode) {
 //       System.out.println("Cat is moving by " + mode);
 //   }
}
