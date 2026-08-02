#pragma once

struct dirent {
    char d_name[129];
};
typedef struct DIR DIR;
DIR* opendir(const char *name);
struct dirent* readdir(DIR* dir);
int closedir(DIR* dir);
