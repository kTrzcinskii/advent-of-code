#include <stdio.h>

int main()
{
    int current, prev, counter = 0, incrs = 0;
    while(scanf("%d", &current))
    {
        if (current < 0) break;
        if (counter == 0) 
        {
            counter++;
            prev = current;
            continue;
        }

        if (current > prev) incrs++;
        prev = current;
        counter++;
    }
    printf("icreases: %d", incrs);

    return 0;
}