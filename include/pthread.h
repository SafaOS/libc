#pragma once

#include <stdint.h>
#include <stddef.h>

typedef uint32_t pthread_t;
typedef void pthread_attr_t;

pthread_t pthread_self(void);
[[noreturn]] void pthread_exit(void *retval);

int pthread_create(pthread_t *thread, const pthread_attr_t *attr, void *(*start_routine)(void *), void *arg);
int pthread_detach(pthread_t thread);
int pthread_join(pthread_t thread, void **retval);

int pthread_attr_init(pthread_attr_t *attr);
int pthread_attr_setstacksize(pthread_attr_t *attr, size_t stacksize);
int pthread_attr_destroy(pthread_attr_t *attr);
