#include <stdio.h>

int main() {
    int number1;
    int result1;

    printf("Enter a number!: ");
    result1 = scanf("%d", &number1);

    if (result1 != 1) {
        printf("Invalid input! Please enter a numeric integer.\n");
        while (getchar() != '\n');
        
        return 1;
    }

    int number2;
    int result2;

    printf("Enter another number!: ");
    result2 = scanf("%d", &number2);

    if (result2 != 1) {
        printf("Invalid input! Please enter a numeric integer.\n");
        while (getchar() != '\n');
        
        return 1;
    }

    int sum = number1 + number2;
    int diff = number1 - number2;
    int product = number1 * number2;
    double quotient = (double)number1 / number2;

    printf("Sum: %d\n", sum);
    printf("Difference: %d\n", diff);
    printf("Product: %d\n", product);
    printf("Quotient: %.2f\n", quotient);

    return 0;
}