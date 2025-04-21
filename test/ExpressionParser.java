class ExpressionParser {
    static class TreeNode {
        char value;
        TreeNode left, right;

        TreeNode(char value) {
            this.value = value;
            this.left = null;
            this.right = null;
        }
    }

    private static boolean isOperator(char c) {
        return c == '+' || c == '-' || c == '*' || c == '/';
    }

    public static TreeNode constructTree(char[] postfix) {
        TreeNode[] stack = new TreeNode[postfix.length];
        int top = -1;

        for (char c : postfix) {
            if (!isOperator(c)) {
                stack[++top] = new TreeNode(c);
            } else {
                TreeNode node = new TreeNode(c);
                node.right = stack[top--];
                node.left = stack[top--];
                stack[++top] = node;
            }
        }
        return stack[top];
    }

    public static char[] repeatSpaces(int count) {
        char[] spaces = new char[count];
        for (int i = 0; i < count; i++) {
            spaces[i] = ' ';
        }
        return spaces;
    }

    public static void printTree(TreeNode root, int level) {
        if (root == null) {
            return;
        }

        printTree(root.right, level + 1);

        char[] spaces = repeatSpaces(level * 4);
        ioTer.printca(spaces);
        char[] val = {root.value};
        ioTer.printca(val);
		ioTer.prints("");

        printTree(root.left, level + 1);
    }

    public static int evaluate(TreeNode root) {
        if (root == null) {
            return 0;
        }

        if (!isOperator(root.value)) {
            return root.value - '0';
        }

        int leftVal = evaluate(root.left);
        int rightVal = evaluate(root.right);

        switch (root.value) {
            case '+': return leftVal + rightVal;
            case '-': return leftVal - rightVal;
            case '*': return leftVal * rightVal;
            case '/': return leftVal / rightVal;
            default: return 0;
        }
    }

    public static void main(String[] args) {
        char[] postfix = { '2', '3', '+', '4', '5', '-', '*' };
        TreeNode root = constructTree(postfix);

        ioTer.prints("Expression Tree:");
        printTree(root, 0);

        int result = evaluate(root);
        ioTer.prints("Result of the Expression:");
        ioTer.printi(result);
    }
}

