#include <stdio.h>
//#include "easy.h"
#include "hard.h"

int main()
{
    char input[100];
    //int total_score_easy = 0;
    int total_score_hard = 0;
    char input1[100];
    char input2[100];
    char input3[100];
    int n = 0;

    while(scanf("%[^\n]%*c", &input))
    {
        if (strcmp(input, "END") == 0) break;
        switch (n)
        {
        case 0:
            strcpy(input1, input);
            break;
        case 1:
            strcpy(input2, input);
            break;
        case 2:
            strcpy(input3, input);
            total_score_hard += calc_score_hard(input1, input2, input3);
            break;
        default:
            break;
        }
        n++;
        n %= 3;
        //total_score_easy += calc_score_easy(input);
    }

    //printf("score easy: %d\n", total_score_easy);
    printf("score hard: %d\n", total_score_hard);
    return 0;
}