# libc
SafaOS's libc written in zig
to compile the lib simply run
```
zig build
```
the lib will be compiled in zig-out/lib/

## Philosophy
I choose zig instead of rust because it is a more low-level language, which is more suitable for a libc.
And I hate C.

### Goals
- Automatically generate the headers from the zig code. (used to do this but it sucked, might need a zig parser for that)
