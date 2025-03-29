#include <assert.h>
#include <stdio.h>

// Here's some comments

// Some explanation to function

void int_pow(int *p_num) {
    int temp = *p_num;
    *p_num = temp * temp;
}

/*
    Some multi comments
*/
int main() {
    printf("Hello, World!");

    int some_num = 25;
    int *p_some_num = &some_num;
    int_pow(p_some_num);
    assert(some_num == 25 * 25);
}
// 13 lines without comments
// 20 with comments. No docs
