#include <stdio.h>
#include "easy.h"
#include <string.h>

int main() 
{
    char input[4];
    int total_score = 0;

    while(scanf("%[^\n]%*c", &input))
    {
        if (strcmp(input, "END") == 0) break;
        total_score += calculate_score_easy(input);
    }

    printf("score: %d\n", total_score);

    return 0;
}