#pragma once
#define isfinite(x)          __builtin_isfinite(x)
#define isgreater(x, y)      __builtin_isgreater(x, y)
#define isgreaterequal(x, y) __builtin_isgreaterequal(x, y)
#define isinf(x)             __builtin_isinf_sign(x)
#define isless(x, y)         __builtin_isless(x, y)
#define islessequal(x, y)    __builtin_islessequal(x, y)
#define islessgreater(x, y)  __builtin_islessgreater(x, y)
#define isnan(x)             __builtin_isnan(x)
#define isnormal(x)          __builtin_isnormal(x)
#define isunordered(x, y)    __builtin_isunordered(x, y)
#define signbit(x)           __builtin_signbit(x)


// Thanks to banan-os https://git.bananymous.com/Bananymous/banan-os/ for all of these constants
#define M_E        2.7182818284590452354
#define M_LOG2E    1.4426950408889634074
#define M_LOG10E   0.43429448190325182765
#define M_LN2      0.69314718055994530942
#define M_LN10     2.30258509299404568402
#define M_PI       3.14159265358979323846
#define M_PI_2     1.57079632679489661923
#define M_PI_4     0.78539816339744830962
#define M_1_PI     0.31830988618379067154
#define M_2_PI     0.63661977236758134308
#define M_2_SQRTPI 1.12837916709551257390
#define M_SQRT2    1.41421356237309504880
#define M_SQRT1_2  0.70710678118654752440

#define HUGE_VAL  __builtin_huge_val()
#define HUGE_VALF __builtin_huge_valf()
#define HUGE_VALL __builtin_huge_vall()
#define INFINITY  __builtin_inff()
#define NAN       __builtin_nanf("")

#define FP_ILOGB0   -2147483647
#define FP_ILOGBNAN +2147483647


int ifloor(double x);
int iceil(double x);

double floor(double x);
double ceil(double x);

double ldexp(double x, int ex);
double frexp(double x, int *ex);

double pow(double x, double y);
double sqrt(double x);
float sqrtf(float x);
double fabs(double x);
double log(double x);

double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);

double sinh(double x);
double cosh(double x);
double tanh(double x);

double asinh(double x);
double acosh(double x);
double atanh(double x);

double sin(double x);
double cos(double x);
double tan(double tan);


double fmod(double x, double y);
float fmodf(float x, float y);
