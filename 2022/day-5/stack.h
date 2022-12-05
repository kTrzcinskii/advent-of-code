#ifndef STACK_H
#define STACK_H
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>


#define N 9
#define MAX_LETTERS 100

typedef struct stack 
{
    int current_index;
    char letters[MAX_LETTERS];
} stack;

void init();
void loadData(stack* st, char* data);
bool isEmpty(stack* st);
void addElementToStack(stack* st, char element);
void removeElementFromStack(stack* st);
char stackTop(stack* st);
void modify_easy(char* input);
void answer();
void modify_hard(char* input);

#endif