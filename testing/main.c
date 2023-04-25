#include "cr.h"
#include "stdlib.h"
#include "unistd.h"
#include "stdio.h"
#include "inttypes.h"

int main(void) {
//    _exit(22);

//    type_t isize = sizeof(int);
//    printf("Call: %"PRId64"\n", isize);

    int32_t call = test_call(101);
    printf("Call: %"PRId32"\n", call);

    return 0;
}

void __wrapper_exit(int32_t code) {
    exit(code);
}

void __wrapper__exit(int32_t code) {
    _exit(code);
}
