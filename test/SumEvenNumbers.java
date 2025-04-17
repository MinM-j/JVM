public class SumEvenNumbers {
    public static void main(String[] args) {
        int sum = 0;
        int count = 0;
        int number = 2;  // Start from the first even number
        
        while (count < 3) {
            sum += number;
            number += 2;  // Move to the next even number
            count++;
        }
		//System.out.println(sum + " " + count + " " + number);

        ioTer.prints("sum of first 3 even numbers starting from 2: ");
        ioTer.printi(sum);
    }
}
