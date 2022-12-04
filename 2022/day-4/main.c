#include <stdio.h>
#include "all.h"

int main()
{
    char input[20];
    int howManyEasy = 0, howManyHard = 0;

    while(scanf("%[^\n]%*c", &input))
    {
        if(strcmp(input, "END") == 0) break;
        if(doesContainEasy(input)) howManyEasy++;
        if(doesOverlapHard(input)) howManyHard++;
    }

    printf("result easy: %d\n", howManyEasy);
    printf("result hard: %d\n", howManyHard);

    return 0;
}