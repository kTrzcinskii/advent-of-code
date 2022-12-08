#include <stdio.h>
#include <stdbool.h>

#define COLUMNS 99
#define ROWS 99

typedef struct visibility
{
    bool left, right, up, down;
} visibility;

typedef struct scenic_score 
{
    int left, right, up, down;
} scenic_score;

bool is_vis(visibility vis)
{
    return vis.down || vis.up || vis.left || vis.right;
}

int main()
{
    int input[ROWS][COLUMNS];
    char input_line[COLUMNS];
    int i, j, k;
    int current;
    visibility is_visible;
    scenic_score score;
    int max_score = 0, current_score;

    int how_many_visible = 0;

    for (i = 0; i < ROWS; i++)
    {
        scanf("%[^\n]%*c", &input_line);
        for (j = 0; j < COLUMNS; j++) input[i][j] = input_line[j] - '0';
    }

    //-------------------------------------------PART 1------------------------------------------
    //from the edge:
    how_many_visible += ROWS * 2 + (COLUMNS - 2) * 2;

    //all the others
    for (i = 1; i < ROWS - 1; i++)
        for (j = 1; j < COLUMNS - 1; j++)
        {
            is_visible.down = true;
            is_visible.up = true;
            is_visible.left = true;
            is_visible.right = true;
            current = input[i][j];

            for (k = i - 1; k >= 0; k--) if (input[k][j] >= current) {is_visible.up = false; break;}
            for (k = i + 1; k < ROWS; k++) if (input[k][j] >= current) {is_visible.down = false; break;}
            for (k = j - 1; k >= 0; k--) if (input[i][k] >= current) {is_visible.left = false; break;}
            for (k = j + 1; k < COLUMNS; k++) if (input[i][k] >= current) {is_visible.right = false; break;}

            if (is_vis(is_visible)) how_many_visible++;
        }
    
    //--------------------------------------------PART 2----------------------------------------
    for (i = 0; i < ROWS; i++)
            for (j = 0; j < COLUMNS; j++)
            {
                current = input[i][j];
                score.left = 0;
                score.right = 0;
                score.up = 0;
                score.down = 0;

                for (k = i - 1; k >= 0; k--)
                {
                    score.up++;
                    if (input[k][j] >= current) break;
                }
                for (k = i + 1; k < ROWS; k++)
                {
                    score.down++;
                    if (input[k][j] >= current) break;
                }
                for (k = j - 1; k >= 0; k--)
                {
                    score.left++;
                    if (input[i][k] >= current) break;
                }
                for (k = j + 1; k < COLUMNS; k++) 
                {
                    score.right++;
                    if (input[i][k] >= current) break;
                }

                current_score = score.down * score.up * score.left * score.right;
                if (current_score > max_score)  max_score = current_score;
            }


    printf("visible: %d\n", how_many_visible);
    printf("max score: %d\n", max_score);
    return 0;
}