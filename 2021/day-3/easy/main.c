#include <stdio.h>
#include <string.h>
#include <math.h>

#define NUM_LEN 12

int calc_val(char binary_num[], int n);

int main()
{
    char input[NUM_LEN + 1];
    char final_number_gamma[NUM_LEN + 1];
    char final_number_epsilon[NUM_LEN + 1];
    int gamma_val, epsilon_val;
    int counter = 0, i;
    //how many '1' is on at given index in list of numbers
    int binary_one[NUM_LEN] = { 0 };

    while(scanf("%[^\n]%*c", &input))
    {
        if (strcmp(input, "end") == 0) break;

        for (i = 0; i < strlen(input); i++)
        {
            if(input[i] == '1') binary_one[i]++;
        }
        counter++;
    }

    for (i = 0; i < NUM_LEN; i++)
    {
        final_number_gamma[i] = binary_one[i] > counter / 2 ? '1' : '0';
        final_number_epsilon[i] = binary_one[i] > counter / 2 ? '0' : '1';
    }
    final_number_gamma[NUM_LEN] = '\0';
    final_number_epsilon[NUM_LEN] = '\0';

    printf("gamma: %s, epsilon: %s\n", final_number_gamma, final_number_epsilon);

    gamma_val = calc_val(final_number_gamma, NUM_LEN);
    epsilon_val = calc_val(final_number_epsilon, NUM_LEN);

    printf("gamma value: %d, epsilon value: %d\n", gamma_val, epsilon_val);

    printf("result: %d", gamma_val * epsilon_val);

    return 0;
}

int calc_val(char binary_num[], int n)
{
    int sum = 0, i;

    for (i = 0 ; i < n; i++)
    {
        if(binary_num[i] == '1') sum += pow(2, n - i - 1);
    }

    return sum;
}