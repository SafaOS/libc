#pragma once
#include <stdlib.h>
#include <stdio.h>
#define STRINGIFY2(x) #x
#define STRINGIFY(x) STRINGIFY2(x)
#define assert(x) \
    x ? 0 : (fputs(__FILE__ ":" STRINGIFY(__LINE__) " Assertion failed: " STRINGIFY(x) "\n", stderr), exit(1))

#if !defined(__cplusplus) && __STDC_VERSION__ >= 201112L && __STDC_VERSION__ < 202311L
	#define static_assert _Static_assert
#endif
