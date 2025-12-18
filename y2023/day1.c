#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DONE -1
#define NEWLINE -2

typedef struct {
    FILE *f;
    char *buff;
    int buff_size;
    int b_idx;
} Ctx;

void assert_eq(int a, int b) {
    printf("%d %d %s\n", a, b, a == b ? "pass" : "fail");
}

bool buff_ends_with(Ctx *ctx, const char *str, int len) {
    if (ctx->b_idx >= len &&
        (strncmp(ctx->buff + ctx->b_idx - len, str, len) == 0)) {
        return true;
    } else {
        return false;
    }
}

// i could just set a max line height tbh. but i wanted to write this by hand
void ensure_size(Ctx *ctx) {
    if (ctx->b_idx >= ctx->buff_size) {
        // starts 0, default 4, 1.5x growth rate
        int new_size = ctx->buff_size == 0 ? 4 : ctx->buff_size * 1.5;
        char *doubled = malloc(new_size * sizeof(int));
        memccpy(doubled, ctx->buff, ctx->buff_size, sizeof(int));
        free(ctx->buff);
        ctx->buff = doubled;
        ctx->buff_size = new_size;
    }
}

void buffer(Ctx *ctx, char ch) {
    ensure_size(ctx);
    ctx->buff[ctx->b_idx++] = ch;
}

int next_int(Ctx *ctx, bool p2) {
    // drains a file char by char looking for (positive) ints. returns -1 when
    // done. returns -2 on newline
    char ch;

    while ((ch = fgetc(ctx->f)) != EOF) {
        if (p2 && ch >= 'a' && ch <= 'z') {
            buffer(ctx, ch);
            if (buff_ends_with(ctx, "zero", 4))
                return 0;
            if (buff_ends_with(ctx, "one", 3))
                return 1;
            if (buff_ends_with(ctx, "two", 3))
                return 2;
            if (buff_ends_with(ctx, "three", 5))
                return 3;
            if (buff_ends_with(ctx, "four", 4))
                return 4;
            if (buff_ends_with(ctx, "five", 4))
                return 5;
            if (buff_ends_with(ctx, "six", 3))
                return 6;
            if (buff_ends_with(ctx, "seven", 5))
                return 7;
            if (buff_ends_with(ctx, "eight", 5))
                return 8;
            if (buff_ends_with(ctx, "nine", 4))
                return 9;
        } else if (ch == '\n') {
            ctx->b_idx = 0;
            return NEWLINE;
        } else if (ch >= '0' && ch <= '9') {
            ctx->b_idx = 0;
            return ch - '0';
        } else {
            ctx->b_idx = 0; // unreachable but lets reset just in case
        }
    }
    return DONE;
}

int solve(bool p2) {
    FILE *f = fopen("day1_input.txt", "r");

    // how to avoid magic numbers like this in c. or is this
    // just how we roll?
    int n;
    int first_num = -1;
    int last_num = -1;
    int sum = 0;

    Ctx ctx = {
        .f = f,
        .buff = malloc(0 * sizeof(int)),
        .buff_size = 0,
        .b_idx = 0,
    };

    while ((n = next_int(&ctx, p2)) != DONE) {
        if (n == NEWLINE) {
            int calibration_number = first_num * 10 + last_num;
            sum += calibration_number;

            first_num = -1;
            last_num = -1;
        } else {
            if (first_num == -1) {
                first_num = n;
            }
            last_num = n;
        }
    }

    fclose(f);

    return sum;
}

int main() {
    assert_eq(solve(false), 55816);
    assert_eq(solve(true), 54980);

    return 0;
}