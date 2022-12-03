#include "easy.h"
#include <stdio.h>

int calc_score_easy(char* input)
{
    int mid = strlen(input) / 2;
    int i, j;
    char same;

    for (i = 0; i < mid; i++)
        for (j = mid; j < strlen(input); j++)
        {
            if (input[i] == input[j]) same = input[i];
        }

    if (same >= 'a' && same <= 'z') return (int)same - lower_case_diff;
    return (int)same - upper_case_diff;
}