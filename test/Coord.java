public class Coord {
    private int x;
    private int y;

    // Constructor
    public Coord(int x, int y) {
        this.x = x;
        this.y = y;
    }

    // Add method
    public void add(Coord other) {
        this.x += other.x;
        this.y += other.y;
    }

    public void display() {
        ioTer.printi(x);
        ioTer.printi(y);
    }

    // Main method to test
    public static void main(String[] args) {
          Coord arr[] = {new Coord(11, 22), new Coord(22,44), new Coord(33,66)};
          Coord sum = new Coord(0,0);
          for(Coord curr: arr){
              sum.add(curr);
          }

          sum.display();
    }
}

