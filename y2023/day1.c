#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "buffer.h"

#define DONE -1
#define NEWLINE -2

typedef struct {
    FILE *f;
    Buff buff;
} Ctx;

void assert_eq(int a, int b) {
    printf("%d %d %s\n", a, b, a == b ? "pass" : "fail");
}

int next_int(Ctx *ctx, bool p2) {
    // drains a file char by char looking for (positive) ints. returns -1 when
    // done. returns -2 on newline
    char ch;

    Buff *buff = &ctx->buff;

    while ((ch = fgetc(ctx->f)) != EOF) {
        if (p2 && ch >= 'a' && ch <= 'z') {
            b_write(buff, ch);
            if (b_ends_with(buff, "zero", 4))
                return 0;
            if (b_ends_with(buff, "one", 3))
                return 1;
            if (b_ends_with(buff, "two", 3))
                return 2;
            if (b_ends_with(buff, "three", 5))
                return 3;
            if (b_ends_with(buff, "four", 4))
                return 4;
            if (b_ends_with(buff, "five", 4))
                return 5;
            if (b_ends_with(buff, "six", 3))
                return 6;
            if (b_ends_with(buff, "seven", 5))
                return 7;
            if (b_ends_with(buff, "eight", 5))
                return 8;
            if (b_ends_with(buff, "nine", 4))
                return 9;
        } else if (ch == '\n') {
            b_clear(buff);
            return NEWLINE;
        } else if (ch >= '0' && ch <= '9') {
            b_clear(buff);
            return ch - '0';
        } else {
            b_clear(buff); // unreachable but lets reset just in case
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

    Ctx ctx = {.f = f,
               .buff = {
                   .buff = NULL,
                   .buff_size = 0,
                   .b_idx = 0,
               }};

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