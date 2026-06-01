#pragma once

#include <stdint.h>
#include <stddef.h>

typedef uint32_t pthread_t;
typedef int pthread_attr_t;
typedef int pthread_mutex_t ;
typedef int pthread_mutexattr_t;
const pthread_mutex_t PTHREAD_MUTEX_INITIALIZER = 0;

const int PTHREAD_CREATE_JOINABLE = 0;
const int PTHREAD_CREATE_DETACHED = 1;

pthread_t pthread_self(void);
[[noreturn]] void pthread_exit(void *retval);

int pthread_create(pthread_t *thread, const pthread_attr_t *attr, void *(*start_routine)(void *), void *arg);
int pthread_detach(pthread_t thread);
int pthread_join(pthread_t thread, void **retval);

int pthread_attr_init(pthread_attr_t *attr);
int pthread_attr_setstacksize(pthread_attr_t *attr, size_t stacksize);
int pthread_attr_setdetachstate(pthread_attr_t *attr, int detachstate);
static inline int pthread_attr_getdetachstate(pthread_attr_t *attr, int *detachstate) {
    // TODO: change when detach state is implemented
    return PTHREAD_CREATE_JOINABLE;
}
int pthread_attr_destroy(pthread_attr_t *attr);

int pthread_mutexattr_init(pthread_attr_t *attr);
int pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attr);
int pthread_mutex_destroy(pthread_mutex_t *mutex);
int pthread_mutex_lock(pthread_mutex_t *mutex);
int pthread_mutex_trylock(pthread_mutex_t *mutex);
int pthread_mutex_unlock(pthread_mutex_t *mutex);
