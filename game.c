#include <stdio.h>
#include <stdlib.h>
#include <conio.h>
#include <windows.h>

#define WIDTH 40
#define HEIGHT 20
#define PADDLE_WIDTH 7
#define BALL_SYMBOL 'O'
#define BRICK_SYMBOL '#'
#define EMPTY_SPACE ' '

typedef struct {
    int x, y;
    int dx, dy;
} Ball;

typedef struct {
    int x, y;
    int width;
} Paddle;

void gotoxy(int x, int y) {
    COORD coord;
    coord.X = x;
    coord.Y = y;
    SetConsoleCursorPosition(GetStdHandle(STD_OUTPUT_HANDLE), coord);
}

void hidecursor() {
    CONSOLE_CURSOR_INFO cursor_info;
    cursor_info.bVisible = 0;
    cursor_info.dwSize = 1;
    SetConsoleCursorInfo(GetStdHandle(STD_OUTPUT_HANDLE), &cursor_info);
}

void draw_paddle(Paddle* paddle) {
    for (int i = 0; i < paddle->width; i++) {
        gotoxy(paddle->x + i, paddle->y);
        putchar('=');
    }
}

void clear_paddle(Paddle* paddle) {
    for (int i = 0; i < paddle->width; i++) {
        gotoxy(paddle->x + i, paddle->y);
        putchar(EMPTY_SPACE);
    }
}

void draw_ball(Ball* ball) {
    gotoxy(ball->x, ball->y);
    putchar(BALL_SYMBOL);
}

void clear_ball(Ball* ball) {
    gotoxy(ball->x, ball->y);
    putchar(EMPTY_SPACE);
}

void draw_bricks(char bricks[HEIGHT][WIDTH]) {
    for (int i = 0; i < HEIGHT / 4; i++) {
        for (int j = 0; j < WIDTH; j++) {
            gotoxy(j, i);
            putchar(bricks[i][j]);
        }
    }
}

int main() {
    hidecursor();

    Ball ball = { WIDTH / 2, HEIGHT / 2, 1, 1 };
    Paddle paddle = { WIDTH / 2 - PADDLE_WIDTH / 2, HEIGHT - 1, PADDLE_WIDTH };
    char bricks[HEIGHT][WIDTH] = { EMPTY_SPACE };

    // Initialize bricks
    for (int i = 0; i < HEIGHT / 4; i++) {
        for (int j = 0; j < WIDTH; j++) {
            bricks[i][j] = BRICK_SYMBOL;
        }
    }

    int score = 0;
    int lives = 3;
    int ch;

    while (1) {
        // Clear the previous positions
        clear_ball(&ball);
        clear_paddle(&paddle);

        // Move ball
        ball.x += ball.dx;
        ball.y += ball.dy;

        // Ball collision with walls
        if (ball.x <= 0 || ball.x >= WIDTH - 1) ball.dx = -ball.dx;
        if (ball.y <= 0) ball.dy = -ball.dy;

        // Ball collision with paddle
        if (ball.y == paddle.y - 1 && ball.x >= paddle.x && ball.x < paddle.x + paddle.width) {
            ball.dy = -ball.dy;
        }

        // Ball collision with bricks
        if (ball.y < HEIGHT / 4 && bricks[ball.y][ball.x] == BRICK_SYMBOL) {
            bricks[ball.y][ball.x] = EMPTY_SPACE;
            ball.dy = -ball.dy;
            score += 10;
        }

        // Ball out of bounds
        if (ball.y >= HEIGHT) {
            lives--;
            ball.x = WIDTH / 2;
            ball.y = HEIGHT / 2;
            ball.dx = 1;
            ball.dy = 1;
            if (lives == 0) {
                gotoxy(WIDTH / 2 - 5, HEIGHT / 2);
                printf("Game Over");
                break;
            }
        }

        // Move paddle
        if (_kbhit()) {
            ch = _getch();
            if (ch == 'a' && paddle.x > 0) paddle.x--;
            if (ch == 'd' && paddle.x < WIDTH - paddle.width) paddle.x++;
        }

        // Draw the new positions
        draw_ball(&ball);
        draw_paddle(&paddle);
        draw_bricks(bricks);
        gotoxy(0, HEIGHT);
        printf("Score: %d  Lives: %d", score, lives);

        Sleep(100); // Adjust this value to slow down the game
    }

    return 0;
}
