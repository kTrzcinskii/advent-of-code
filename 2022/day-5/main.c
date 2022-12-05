#include <stdio.h>
#include "stack.h"

int main()
{
    char input[20];

    init();

    while(scanf("%[^\n]%*c", &input))
    {
        if (strcmp(input, "END") == 0) break;
        //modify_easy(input);
        modify_hard(input);
    }

    answer();

    return 0;
}