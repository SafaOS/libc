const malloc_init = @import("stdlib.zig").__malloc__init__;
const sys = @import("sys/root.zig");
const OsStr = sys.raw.OsStr;
const Slice = sys.raw.Slice;

export var __lib__argc: usize = 0;
export var __lib__argv: ?[*]const Slice(u8) = null;
export fn __lib__init__(argc: usize, argv: [*]const Slice(u8)) void {
    __lib__argc = argc;
    __lib__argv = argv;
    malloc_init();
}

pub fn __lib__argv_get() [*]const Slice(u8) {
    return __lib__argv.?;
}

pub fn __lib__argc_get() usize {
    return __lib__argc;
}
