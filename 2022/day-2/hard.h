#ifndef HARD_H
#define HARD_H

typedef enum {
    ROCK_E = 'A',
    PAPER_E = 'B',
    SCISSORS_E = 'C'
} enemy_input;

typedef enum {
    LOST_P = 'X',
    DRAW_P = 'Y',
    WIN_P = 'Z'
} expected_output;

typedef enum {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
} shape_value;

typedef enum {
    LOST = 0,
    DRAW = 3,
    WIN = 6,
} outcome_value;

outcome_value expected_to_val(expected_output x);

shape_value enemy_sign_to_value(enemy_input sign);

int calculate_score_hard(char* input);

#endif