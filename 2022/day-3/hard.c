#include "hard.h"
#include <stdio.h>

int calc_score_hard(char* input1, char* input2, char* input3)
{
    int i,j,k;
    char same;
    
    for (i = 0; i < strlen(input1); i++)
        for (j = 0; j < strlen(input2); j++)
            for (k = 0; k < strlen(input3); k++)
                if (input1[i] == input2[j] && input1[i] == input3[k]) same = input1[i];
    
    if (same >= 'a' && same <= 'z') return (int)same - lower_case_diff;
    return (int)same - upper_case_diff;
}