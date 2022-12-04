#ifndef ALL_H
#define ALL_H

#include <string.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct 
{
    int x, y;
} pair;

int indexOf(char c, char* input);

void cpyNumToString(char* dest, char* src, int len, int nullPos);

bool doesContainEasy(char* input);

void load(pair* first, pair* second, char* input);

bool doesOverlapHard(char* input);

#endif