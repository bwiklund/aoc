#include <stdbool.h>
#include <stdio.h>

#define DONE -1
#define NEWLINE -2

void assert_eq(int a, int b) {
    printf("%d %d %s\n", a, b, a == b ? "pass" : "fail");
}

int next_int(FILE *f) {
    // drains a file char by char looking for (positive) ints. returns -1 when
    // done. returns -2 on newline
    char ch;
    while ((ch = fgetc(f)) != EOF) {
        if (ch == '\n') {
            return NEWLINE;
        }
        if (ch >= 48 && ch <= 57) { // is digit
            return ch - 48;
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

    assert_eq(sum, 55816);

    return 0;
}