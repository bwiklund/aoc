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

#define DIGIT_COUNT 10
static const char *strs[DIGIT_COUNT] = {"zero",  "one",  "two", "three",
                                        "four",  "five", "six", "seven",
                                        "eight", "nine"};

int next_int(Ctx *ctx, bool p2) {
    // drains a file char by char looking for (positive) ints. returns -1 when
    // done. returns -2 on newline
    char ch;
    Buff *buff = &ctx->buff;
    while ((ch = fgetc(ctx->f)) != EOF) {
        if (p2 && ch >= 'a' && ch <= 'z') {
            b_write(buff, ch);
            for (int i = 0; i < DIGIT_COUNT; i++) {
                if (b_ends_with(buff, strs[i])) {
                    return i;
                }
            }
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

    Ctx ctx = {.f = f, .buff = b_new()};

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