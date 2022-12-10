#include "signal.h"

commands check_command(char* input)
{
    if (strcmp(input, "noop") == 0) return NOOP;

    return ADDX;
}

int how_many_add(char* input)
{
    input += strlen("addx ");

    return atoi(input);
}

bool is_wanted_cycle(int cycle, int* cycles, int n)
{
    bool wanted_cycle = false;
    int i;
    for (i = 0; i < n; i++) 
        if (cycles[i] == cycle)
        {
            wanted_cycle = true;
            break;
        }
    return wanted_cycle;
}

int add_to_signal_strength(cpu_t cpu, int* cycles, int n)
{
    bool wanted_cycle = is_wanted_cycle(cpu.cycle, cycles, n);
    if (!wanted_cycle) return 0;
    return cpu.cycle * cpu.registerX;
}

char get_draw_symbol(int sprite_position, int cycle, int* cycles, int n)
{
    if (cycle >= sprite_position - 1 && cycle <= sprite_position + 1) return '#';
    return '_';
}

void draw(char* symbols, int j, int* cycles, int n)
{
    int i;
    bool is_wanted;
    for (i = 0; i < j; i++) 
    {
        printf("%c",symbols[i]);
        is_wanted = is_wanted_cycle(i, cycles, n);
        if (is_wanted) printf("\n");
    }
}

int get_sprite_position(cpu_t cpu, int cycles_in_one_row)
{
    return cpu.registerX + ((cpu.cycle) / (cycles_in_one_row) * cycles_in_one_row);
}
