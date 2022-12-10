#include "signal.h"

#define CYCLES_NUM 6
#define ALL_CYCLES 240
#define CYCLES_IN_ONE_ROW 40

int main()
{
    cpu_t cpu;
    cpu.cycle = 0;
    cpu.registerX = 1;
    char input[10];
    commands current_command;
    int total_signal_strength = 0;
    int cycles[CYCLES_NUM] = {20, 60, 100, 140, 180, 220};
    int other_cycles[CYCLES_NUM] = {39, 79, 119, 159, 199, 239};
    char symbols[ALL_CYCLES + 1] = {'\0'};
    char sym;
    int sprite_position = cpu.registerX;

    while(scanf("%[^\n]%*c", &input))
    {
        if(strcmp(input, "end") == 0) break;
        sprite_position = get_sprite_position(cpu, CYCLES_IN_ONE_ROW);
        current_command = check_command(input);
        symbols[cpu.cycle] = get_draw_symbol(sprite_position, cpu.cycle, cycles, CYCLES_NUM);
        cpu.cycle++;
        total_signal_strength += add_to_signal_strength(cpu, cycles, CYCLES_NUM);
        if (current_command == ADDX) 
        {
            sprite_position = get_sprite_position(cpu, CYCLES_IN_ONE_ROW);
            symbols[cpu.cycle] = get_draw_symbol(sprite_position, cpu.cycle, cycles, CYCLES_NUM);
            cpu.cycle++;
            total_signal_strength += add_to_signal_strength(cpu, cycles, CYCLES_NUM);
            cpu.registerX += how_many_add(input);
        }
    }

    printf("total signal strength: %d\n", total_signal_strength);

    draw(symbols, ALL_CYCLES, other_cycles, CYCLES_NUM);

    return 0;
}