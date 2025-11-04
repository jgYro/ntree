public class TestSample {

    public static void simpleMethod() {
        System.out.println("Hello Java");
    }

    public static int methodWithIf(int x) {
        if (x > 0) {
            return x + 1;
        } else {
            return 0;
        }
    }

    public static int complexMethod(int a, int b) {
        if (a > b) {
            if (a > 10) {
                return a * 2;
            } else {
                return a + b;
            }
        } else {
            if (b > 10) {
                return b * 3;
            } else {
                return a - b;
            }
        }
    }

    private int calculateSomething(int value) {
        switch (value) {
            case 1:
                return value * 2;
            case 2:
                return value * 3;
            default:
                return value;
        }
    }
}