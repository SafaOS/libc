#pragma once
#include "types.h"

int mkdir(const char *path, mode_t mode);
struct stat {
    mode_t st_mode;
    off_t  st_size;
    time_t st_atime;
    time_t st_mtime;
    time_t st_ctime;
};




#define S_IRWXU 0000700    /* RWX mask for owner */
#define S_IRUSR 0000400    /* R for owner */
#define S_IWUSR 0000200    /* W for owner */
#define S_IXUSR 0000100    /* X for owner */

#define S_IRWXG 0000070    /* RWX mask for group */
#define S_IRGRP 0000040    /* R for group */
#define S_IWGRP 0000020    /* W for group */
#define S_IXGRP 0000010    /* X for group */

#define S_IRWXO 0000007    /* RWX mask for other */
#define S_IROTH 0000004    /* R for other */
#define S_IWOTH 0000002    /* W for other */
#define S_IXOTH 0000001    /* X for other */


#define S_IFMT   0170000 // Bit mask for the file type bit field
#define S_IFDIR  0040000 // File type value for a directory
#define S_IFREG  0100000 // Regular file
#define S_IFCHR  0020000 // Device file (TODO: block devices)

#define S_ISDIR(m) (((m) & S_IFMT) == S_IFDIR)


int stat(const char * path,
         struct stat * statbuf);

static int lstat(const char *path,
         struct stat *statbuf) {
             // TODO: symlink stat
             return stat(path, statbuf);
}
