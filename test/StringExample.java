public class StringExample {
    public static void main(String[] args) {
        String firstMainString = "This is the first main string."; // 2
        //ioTer.prints(firstMainString);
        createStrings();
        String secondMainString = "This is the last main string."; //8
        String lol = "This is the last main string."; //8
        String loloill = "This is the last main string."; //8
        //ioTer.prints(secondMainString);
    }

    public static void anotherString(){
          String firstString = "First string from createStrings()"; // 4
          String secondString = "Second string from createStrings()"; // 6
    }
    public static void createStrings() {
        String firstString = "First string from createStrings()"; // 4
        anotherString();
        String secondString = "Second string from createStrings()"; // 6
        //ioTer.prints(firstString);
        //ioTer.prints(secondString);
    }
}
