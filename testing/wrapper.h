#pragma once

#include "inttypes.h"

void* __wrapper_malloc(size_t size);

void __wrapper_exit(int32_t code);
void __wrapper__exit(int32_t code);
