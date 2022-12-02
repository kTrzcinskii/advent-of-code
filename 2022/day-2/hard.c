#include "hard.h"

outcome_value expected_to_val(expected_output x)
{
    switch (x)
    {
    case LOST_P:
        return LOST;
        break;
    case DRAW_P:
        return DRAW;
        break;
    case WIN_P:
        return WIN;
        break;
    default:
        break;
    }
}

shape_value enemy_sign_to_value(enemy_input sign)
{
    switch (sign)
    {
    case ROCK_E:
        return ROCK;
        break;
    case PAPER_E:
        return PAPER;
    case SCISSORS_E:
        return SCISSORS;
    default:
        break;
    }
}

int calculate_score_hard(char* input)
{
    char enemy_sign = input[0];
    char expected_output = input[2];
    int score = 0;
    outcome_value result = expected_to_val(expected_output);
    shape_value player_val;
    shape_value enemy_val = enemy_sign_to_value(enemy_sign);

    if (result == DRAW) 
    {
        player_val = enemy_val;
    } else if (result == LOST)
    {
        player_val = enemy_val - 1;
        if (player_val == 0) player_val = SCISSORS;
    } else 
    {
        player_val = enemy_val + 1;
        if (player_val == 4) player_val = ROCK;
    }

    score = player_val + result;
    return score;
}