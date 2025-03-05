public class MultiDimensionalArrayExample {

    public static void main(String[] args) {
        // Creating a 2D array of doubles (3x3 for example)
        double[][] matrix = {
            {1.1, 2.2, 3.3},
            {4.4, 5.5, 6.6},
            {7.7, 8.8, 9.9}
        };

        // Creating a 1D array of doubles to store the sum of each row
        double[] rowSums = new double[matrix.length];

        // Loop through each row of the 2D array and calculate the sum
        for (int i = 0; i < matrix.length; i++) {
            double sum = 0;  // Variable to store the sum of the current row
            for (int j = 0; j < matrix[i].length; j++) {
                sum += matrix[i][j];  // Add each element of the row to the sum
            }
            rowSums[i] = sum;  // Store the sum of the row in the 1D array
        }
		ioTer.prints("Sum of rows:");
		for (int i = 0; i < rowSums.length; i++){
			ioTer.printn(rowSums[i]);
		}
    }
}

