# salibc
SafaOS's libc written in zig

## Building
first you need to run the initialization script to compile the `safa-api` library.
```
./init.sh
```

you can build the libc with
```
./build.sh
```

you will then find the libc (`libsalibc.a`) and the api (`libsafa_api.a`) libraries in the `out` directory,
you need to link with them both because the libc depends on the api.

## Philosophy
I choose zig instead of rust because it is a more low-level language, which is more suitable for a libc.
And I hate C.
