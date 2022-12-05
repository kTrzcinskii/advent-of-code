#include "stack.h"

stack stacks[N];

stack toMove;

void loadData(stack* st, char* data)
{
    for (; *data != '\0'; data++) addElementToStack(st, *data);
}

void init()
{
    //hardcoded stack values, there is no sense in trying to load it from console or txt
    char* s1 = "FHBVRQDP";
    char* s2 = "LDZQWV";
    char* s3 = "HLZQGRPC";
    char* s4 = "RDHFJVB";
    char* s5 = "ZWLC";
    char* s6 = "JRPNTGVM";
    char* s7 = "JRLVMBS";
    char* s8 = "DPJ";
    char* s9 = "DCNWV";
    int i;
    for (i = 0; i < N; i++)
    {
        stacks[i].current_index = -1;
        stacks[i].letters[0] = '\0';
    }

    toMove.current_index = -1;
    toMove.letters[0] = '\0';

    loadData(&stacks[0], s1);
    loadData(&stacks[1], s2);
    loadData(&stacks[2], s3);
    loadData(&stacks[3], s4);
    loadData(&stacks[4], s5);
    loadData(&stacks[5], s6);
    loadData(&stacks[6], s7);
    loadData(&stacks[7], s8);
    loadData(&stacks[8], s9);
}

bool isEmpty(stack* st)
{
    return st->current_index == -1;
}

char stackTop(stack *st)
{
    return st->letters[st->current_index];
}

void addElementToStack(stack *st, char element)
{
    if (st->current_index + 1 > MAX_LETTERS - 1) return;
    st->current_index++;
    st->letters[st->current_index] = element;
}

void removeElementFromStack(stack* st)
{
    if (isEmpty(st)) return;
    st->letters[st->current_index] = '\0';
    st->current_index--;
}

void answer()
{
    char final[N];
    int i;

    for (i = 0; i < N; i++) final[i] = stackTop(&stacks[i]);

    printf("Final stack: %s", final);
}

void modify_easy(char* input)
{
    int howMany = 0, from = -1, to = -1, i;
    char c;
    sscanf(input, "move %d from %d to %d", &howMany, &from, &to);
    for (i = 0; i < howMany; i++)
    {
        if(isEmpty(&stacks[from - 1])) break;
        c = stackTop(&stacks[from - 1]);
        removeElementFromStack(&stacks[from - 1]);
        addElementToStack(&stacks[to - 1], c);
    }
}

void modify_hard(char* input)
{
    int howMany = 0, from = -1, to = -1, i;
    sscanf(input, "move %d from %d to %d", &howMany, &from, &to);

    for (i = 0; i < howMany; i++)
    {
        if(isEmpty(&stacks[from - 1])) break;
        addElementToStack(&toMove, stackTop(&stacks[from - 1]));
        removeElementFromStack(&stacks[from - 1]);
    }

    for (i = 0; i < howMany; i++) 
    {
        addElementToStack(&stacks[to - 1], stackTop(&toMove));
        removeElementFromStack(&toMove);
    }
}