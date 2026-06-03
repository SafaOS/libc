#pragma once

#include <stdint.h>
#include <stddef.h>
#include <time.h>

typedef uint32_t pthread_t;
typedef int pthread_attr_t;
typedef int pthread_mutex_t ;
typedef int pthread_mutexattr_t;
#define PTHREAD_MUTEX_INITIALIZER 0

#define PTHREAD_CREATE_JOINABLE 0
#define PTHREAD_CREATE_DETACHED 1

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

typedef int pthread_cond_t;
typedef int pthread_condattr_t;

#define PTHREAD_COND_INITIALIZER  0

int pthread_cond_init(pthread_cond_t *cond, pthread_condattr_t *cond_attr);
int pthread_cond_signal(pthread_cond_t *cond);
int pthread_cond_broadcast(pthread_cond_t *cond);
int pthread_cond_wait(pthread_cond_t *cond, pthread_mutex_t *mutex);
int pthread_cond_timedwait(pthread_cond_t *cond, pthread_mutex_t *mutex, const struct timespec *abstime);
int pthread_cond_destroy(pthread_cond_t *cond);

int pthread_condattr_init(pthread_condattr_t *attr);
int pthread_condattr_destroy(pthread_condattr_t *attr);
