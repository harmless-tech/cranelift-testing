#include "stdlib.h"
#include "unistd.h"

#include "wrapper.h"

void* __wrapper_malloc(size_t size) {
    return malloc(size);
}

void __wrapper_exit(int32_t code) {
    exit(code);
}

void __wrapper__exit(int32_t code) {
    _exit(code);
}
