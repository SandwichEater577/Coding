#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int main() {

    int age = 25;
    // %d use for integers
    printf("I am %d years old.\n", age);

    float height = 1.86f;
    // %f use for floating-point numbers
    // specify decimals with %.nf
    printf("I am %.2f meters tall.\n", height);

    char grade = 'A';
    // %c use for characters
    printf("My grade is %c.\n", grade);

    char name[] = "Michael";
    // %s use for strings
    printf("My name is %s.\n", name);

    // boolean example
    bool isAdult = (age >= 18);
    printf("Is adult: %s\n", isAdult ? "true" : "false");

    // formatted output with width and padding
    printf("Age padded: %02d\n", age); // prints with leading zeros if needed

    return 0;
}

/*
Example output:
I am 25 years old.
I am 1.86 meters tall.
My grade is A.
My name is Michael.
Name length: 7
Is adult: true
Age padded: 025
*/