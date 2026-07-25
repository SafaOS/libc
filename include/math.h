#pragma once


#define fpclassify(x)        __builtin_fpclassify(FP_NAN, FP_INFINITE, FP_NORMAL, FP_SUBNORMAL, FP_ZERO, x)
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

#define FP_INFINITE  0
#define FP_NAN       1
#define FP_NORMAL    2
#define FP_SUBNORMAL 3
#define FP_ZERO      4

#define FP_ILOGB0   -2147483647
#define FP_ILOGBNAN +2147483647


int ifloor(double x);
int iceil(double x);
int iceilf(float x);

double floor(double x);
float floorf(float x);
double ceil(double x);

float ceilf(float x);

double ldexp(double x, int ex);
double frexp(double x, int *ex);
double exp(double x);
float  expf(float x);

double pow(double x, double y);
float  powf(float x, float y);
double sqrt(double x);
float sqrtf(float x);
double fabs(double x);
float fabsf(float x);

double log(double x);
float  logf(float x);


double log10(double x);
float  log10f(float x);

float asinf(float x);
float acosf(float x);
float atanf(float x);
float atan2f(float y, float x);

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

float sinhf(float x);
float coshf(float x);
float tanhf(float x);

float asinhf(float x);
float acoshf(float x);
float atanhf(float x);

double sin(double x);
double cos(double x);
double tan(double tan);

float sinf(double x);
float cosf(double x);
float tanf(double tan);

double fmod(double x, double y);
float fmodf(float x, float y);

double modf(double, double*);
float modff(float, float*);

double fmax(double x, double y);
double fmin(double x, double y);

float fmaxf(float x, float y);
float fminf(float x, float y);

float hypotf(float, float);
float erff(float);
float copysignf(float, float);

double hypot(double, double);
double erf(double);
double copysign(double, double);

double j0(double x);
double j1(double x);
double jn(int n, double x);

float j0f(float x);
float j1f(float x);
float jnf(int n, float x);

long double j0l(long double x);
long double j1l(long double x);
long double jnl(int n, long double x);

double      scalbn( double arg, int exp );
float       scalbnf( float arg, int exp );


long lrintf ( float num );
long lrint ( double num );
