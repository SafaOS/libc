#pragma once
#include <ctype.h>
#include <string.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>


typedef int mbstate_t;

typedef int wctype_t;

typedef __WINT_TYPE__ wint_t;

#define WCHAR_MIN	__WCHAR_MIN__
#define WCHAR_MAX	__WCHAR_MAX__
#define WEOF		((wchar_t)-1)
