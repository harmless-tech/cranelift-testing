#include "stdio.h"
#include "inttypes.h"

#include "cr.h"

int main(void) {
//    size_t mem = ___std_malloc(10);
//    printf("Call: %"PRId64"\n", (uint64_t) mem);

    int32_t call = test_call(101);
    printf("Call: %"PRId32"\n", call);

    return 0;
}
