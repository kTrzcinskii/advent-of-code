#include <stdio.h>
//#include "easy.h"
#include "hard.h"
#include <string.h>

int main() 
{
    char input[4];
    //int total_score_easy = 0;
    int total_score_hard = 0;

    while(scanf("%[^\n]%*c", &input))
    {
        if (strcmp(input, "END") == 0) break;
        //total_score_easy += calculate_score_easy(input);
        total_score_hard += calculate_score_hard(input);
    }

    //printf("score_easy: %d\n", total_score_easy);
    printf("score_hard: %d\n", total_score_hard);

    return 0;
}