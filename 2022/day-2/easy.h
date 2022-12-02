#ifndef EASY_H
#define EASY_H

typedef enum {
    ROCK_E = 'A',
    PAPER_E = 'B',
    SCISSORS_E = 'C'
} enemy_input;

typedef enum {
    ROCK_P = 'X',
    PAPER_P = 'Y',
    SCISSORS_P = 'Z'
} player_input;

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

shape_value player_sign_to_value(player_input sign);
shape_value enemy_sign_to_value(enemy_input sign);

int calculate_score_easy(char* input);

#endif