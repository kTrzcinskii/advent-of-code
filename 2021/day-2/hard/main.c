#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LEN 10

typedef struct position
{
    int horizontal;
    int depth;
    int aim;
} position;

int find_space_index(char* input);

int main()
{
    position pos;
    char* input = malloc(sizeof(char) * MAX_LEN);
    int index, number;

    pos.depth = 0;
    pos.horizontal = 0;
    pos.aim = 0;

    while(scanf("%[^\n]%*c", input))
    {
        if (strcmp(input, "end") == 0) break;
        index = find_space_index(input);
        number = input[index + 1] - 48;
        switch (input[0])
        {
        case 'f':
            pos.horizontal += number;
            pos.depth += pos.aim * number;
            break;        
        case 'd':
            pos.aim += number;
            break;        
        case 'u':
            pos.aim -= number;
            break;        
        }
    };

    printf("res: %d", pos.depth * pos.horizontal);

    return 0;
}

int find_space_index(char* input)
{
    int i;
    for (i = 0; i < MAX_LEN; i++) if (input[i] == ' ') return i;
}