#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#define DONE -1
#define NEWLINE -2

void assert_eq(int a, int b) {
    printf("%d %d %s\n", a, b, a == b ? "pass" : "fail");
}

bool buff_ends_with(char *buff, int *b_idx, const char *str, int len) {
    if (*b_idx >= len && (strncmp(buff + *b_idx - len, str, len) == 0)) {
        // *b_idx = 0;
        return true;
    } else {
        return false;
    }
}

int next_int(FILE *f) {
    // drains a file char by char looking for (positive) ints. returns -1 when
    // done. returns -2 on newline
    char ch;

    static char buff[100000];
    static int b_idx = 0;

    while ((ch = fgetc(f)) != EOF) {
        if (ch >= 'a' && ch <= 'z') {
            buff[b_idx++] = ch;
            if (buff_ends_with(buff, &b_idx, "zero", 4))
                return 0;
            if (buff_ends_with(buff, &b_idx, "one", 3))
                return 1;
            if (buff_ends_with(buff, &b_idx, "two", 3))
                return 2;
            if (buff_ends_with(buff, &b_idx, "three", 5))
                return 3;
            if (buff_ends_with(buff, &b_idx, "four", 4))
                return 4;
            if (buff_ends_with(buff, &b_idx, "five", 4))
                return 5;
            if (buff_ends_with(buff, &b_idx, "six", 3))
                return 6;
            if (buff_ends_with(buff, &b_idx, "seven", 5))
                return 7;
            if (buff_ends_with(buff, &b_idx, "eight", 5))
                return 8;
            if (buff_ends_with(buff, &b_idx, "nine", 4))
                return 9;
        } else if (ch == '\n') {
            // b_idx = 0;
            return NEWLINE;
        } else if (ch >= '0' && ch <= '9') {
            // b_idx = 0;
            return ch - '0';
        } else {
            // b_idx = 0; // unreachable but lets reset just in case
        }
    }
    return DONE;
}

int main() {
    FILE *f = fopen("day1_input.txt", "r");

    // how to avoid magic numbers like this in c. or is this
    // just how we roll?
    int n;
    int first_num = -1;
    int last_num = -1;
    int sum = 0;

    while ((n = next_int(f)) != DONE) {
        if (n == NEWLINE) {
            printf("\n %d %d\n", first_num, last_num);
            int calibration_number = first_num * 10 + last_num;
            sum += calibration_number;

            first_num = -1;
            last_num = -1;
        } else {
            printf("%d ", n);
            if (first_num == -1) {
                first_num = n;
            }
            last_num = n;
        }
    }

    fclose(f);

    assert_eq(sum, 55816);

    return 0;
}