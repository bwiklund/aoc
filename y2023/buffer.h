#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

// a dynamic array implementation
typedef struct {
    char *buff;
    int buff_size;
    int b_idx;
} Buff;

bool b_ends_with(Buff *buff, const char *str, int len) {
    if (buff->b_idx >= len &&
        (strncmp(buff->buff + buff->b_idx - len, str, len) == 0)) {
        return true;
    } else {
        return false;
    }
}

// i could just set a max line height tbh. but i wanted to write this by hand
void b_ensure_size(Buff *buff) {
    if (buff->b_idx >= buff->buff_size) {
        // starts 0, default 4, 1.5x growth rate
        int new_size = buff->buff_size == 0 ? 4 : buff->buff_size * 1.5;
        char *doubled = malloc(new_size * sizeof(char));
        if (buff->buff != 0) {
            memccpy(doubled, buff->buff, buff->buff_size, sizeof(int));
            free(buff->buff);
        }
        buff->buff = doubled;
        buff->buff_size = new_size;
    }
}

void b_write(Buff *buff, char ch) {
    b_ensure_size(buff);
    buff->buff[buff->b_idx++] = ch;
}

void b_clear(Buff *buff) { buff->b_idx = 0; }