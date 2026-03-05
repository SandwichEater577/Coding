#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    printf("Simple Calculator\n");
    while (1 == 1)
    {
        printf("Enter an expression (or 'exit' to quit): ");
        char input[100];
        fgets(input, sizeof(input), stdin);
        if (strcmp(input, "exit\n") == 0)
        {
            break;
        }
        else if (strcmp(input, "num\n") == 0)
        {
            printf("Enter a number: ");
            char numInput[100];
            fgets(numInput, sizeof(numInput), stdin);
            double num = atof(numInput);
            printf("You entered: %lf\n", num);
        }
    }
    return 0;
}