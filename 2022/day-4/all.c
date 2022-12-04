#include "all.h"

int indexOf(char c, char* input)
{
    int index = -1, i;

    for(i = 0; *(input + i) != '\0'; i++)
    {
        if (*(input + i) == c) 
        {
            index = i;
            break;
        }
    }

    return index;
}

void cpyNumToString(char* dest, char* src, int len, int nullPos)
{
    strncpy(dest, src, len);
    dest[nullPos] = '\0';
}

void load(pair* first, pair* second, char* input)
{
    int middle = indexOf(',', input);
    int firstDash = indexOf('-', input);
    int secondDash = middle + indexOf('-', (input + middle));

    char num1[4], num2[4], num3[4], num4[4];
    cpyNumToString(num1, input, firstDash, firstDash);  
    cpyNumToString(num2, input + firstDash + 1, middle - firstDash, middle - firstDash - 1);   
    cpyNumToString(num3, input + middle + 1, secondDash - middle, secondDash - middle - 1);
    cpyNumToString(num4, input + secondDash + 1, strlen(input) - secondDash, strlen(input) - secondDash - 1);

    first->x = atoi(num1);
    first->y = atoi(num2);
    second->x = atoi(num3);
    second->y = atoi(num4);
}

//easy part
bool doesContainEasy (char* input)
{
    pair first, second;

    load(&first, &second, input);

    if ((first.x >= second.x && first.y <= second.y) || (second.x >= first.x && second.y <= first.y)) return true;

    return false;
}

//hard part
bool doesOverlapHard(char* input)
{
    pair first, second;

    load(&first, &second, input);

    if ((first.x >= second.x && first.x <= second.y) || (first.y >= second.x && first.y <= second.y) || (second.x >= first.x && second.x <= first.y) || (second.y >= first.x && second.y <= first.y)) return true;

    return false;
}