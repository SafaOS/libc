const ffi = @import("ffi.zig");
const Optional = ffi.Optional;

pub const AbiStructures = extern struct {
    stdio: TaskStdio,
};

pub const TaskStdio = extern struct {
    stdout: Optional(usize),
    stdin: Optional(usize),
    stderr: Optional(usize),
};
