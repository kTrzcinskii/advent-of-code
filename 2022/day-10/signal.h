#ifndef SIGNAL_H
#define SIGNAL_H

#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>

typedef enum 
{
    ADDX,
    NOOP
} commands;

typedef struct 
{
    int cycle;
    int registerX;
} cpu_t;

commands check_command(char* input);

int how_many_add(char* input);

int add_to_signal_strength(cpu_t cpu, int* cycles, int n);

char get_draw_symbol(int sprite_position, int cycle, int* cycles, int n);

void draw(char* symbols, int j, int* cycles, int n);

int get_sprite_position(cpu_t cpu, int cycles_in_one_row);

#endif