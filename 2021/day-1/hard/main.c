#include <stdio.h>
#include <stdbool.h>

typedef struct measurement_window 
{
    int a,b,c;
} measurement_window;

int sum(measurement_window w);

void clear(measurement_window *w);

bool compare(measurement_window w1, measurement_window w2);

void switch_w(measurement_window *w1, measurement_window *w2);

int main()
{
    measurement_window w1, w2;

    bool cmpr;
    int a, counter = 0, incrs = 0;

    clear(&w1);
    clear(&w2);

    while(scanf("%d", &a))
    {
        if (a < 0) break;
        switch (counter)
        {
        case 0:
            w1.a = a;
            counter++;
            break;
        case 1:
            w1.b = a;
            w2.a = a;
            counter++;
            break;
        case 2:
            w1.c = a;
            w2.b =a;
            counter++;
            break;
        case 3:
            w2.c = a;
            cmpr = compare(w1, w2);
            if (cmpr) incrs++;
            switch_w(&w1, &w2);
            break;
        }
    }

    printf("increases: %d", incrs);

    return 0;
}

int sum (measurement_window w)
{
    return w.a + w.b + w.c;
}

bool compare(measurement_window w1, measurement_window w2)
{
    return sum(w2) > sum(w1);
}

void clear(measurement_window* w)
{
    w->a = 0;
    w->b = 0;
    w->c = 0;
}

void switch_w(measurement_window *w1, measurement_window *w2)
{
    w1->a = w2->a;
    w1->b = w2->b;
    w1->c = w2->c;
    w2->a = w2->b;
    w2->b = w2->c;
}