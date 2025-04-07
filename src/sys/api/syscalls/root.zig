pub const io = @import("io.zig");
const raw = @import("raw.zig");

pub fn exit(code: usize) noreturn {
    raw.sysexit(code);
}
