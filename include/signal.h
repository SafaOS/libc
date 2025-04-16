#pragma once

typedef void (*sighandler_t)(int);
#define SIG_IGN NULL
#define SIG_DFL NULL
#define sig_atomic_t int
#define SIGINT 2

sighandler_t signal(int signum, sighandler_t handler);
