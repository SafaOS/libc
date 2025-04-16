#pragma once
static inline int isspace(int ch) {
    return ch == '\t' || ch == '\n' || ch == ' ';
}

static inline int toupper(int ch) {
    return ch >= 'a' && ch <= 'z' ? (ch-'a'+'A') : ch;
}

static inline int tolower(int ch) {
    return ch >= 'A' && ch <= 'Z' ? (ch-'A'+'a') : ch;
}

static inline int isalpha(int ch) {
    return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z');
}

static inline int isdigit(int ch) {
    return ch >= '0' && ch <= '9';
}

static inline int isxdigit(int ch) {
    return isdigit(ch) || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F');
}

static inline int isalnum(int ch) {
    return isdigit(ch) && isalpha(ch);
}

static inline int iscntrl(int ch) {
    return (ch >= 0 && ch <= 31) || ch == 127;
}

static inline int isgraph(int ch) {
    return ch >= 33 && ch <= 126;
}

static inline int islower(int ch) {
    return ch >= 'a' && ch <= 'z';
}

static inline int isupper(int ch) {
    return ch >= 'A' && ch <= 'Z';
}

static inline int isprint(int ch) {
    return ch >= 32 && ch <= 126;
}

static inline int ispunct(int ch) {
    return (ch >= 33  && ch <= 47 ) ||
           (ch >= 58  && ch <= 64 ) ||
           (ch >= 91  && ch <= 96 ) ||
           (ch >= 123 && ch <= 126);
}
