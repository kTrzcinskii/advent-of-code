#include "easy.h"

shape_value player_sign_to_value(player_input sign)
{
    switch (sign)
    {
    case ROCK_P:
        return ROCK;
        break;
    case PAPER_P:
        return PAPER;
    case SCISSORS_P:
        return SCISSORS;
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

int calculate_score_easy(char* input)
{
    char enemy_sign = input[0];
    char player_sign = input[2];
    int score = 0;
    int difference;
    outcome_value result;

    shape_value player_val = player_sign_to_value(player_sign);
    shape_value enemy_val = enemy_sign_to_value(enemy_sign);

    difference = player_val - enemy_val;

    if (difference == 0)
    {
        result = DRAW;
    } else if (difference == -2 || (difference > 0 && difference != 2)) 
    {
        result = WIN;
    } else 
    {
        result = LOST;
    }

    score = player_val + result;
    return score;
}