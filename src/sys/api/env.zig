const abi = @import("../abi/root.zig");
const RawSliceConst = abi.ffi.RawSliceConst;
const Optional = abi.ffi.Optional;

extern fn sysenv_get(key: RawSliceConst(u8)) Optional(RawSliceConst(u8));
extern fn sysenv_set(key: RawSliceConst(u8), value: RawSliceConst(u8)) void;
extern fn sysenv_remove(key: RawSliceConst(u8)) void;

/// Get the value of an environment variable.
pub fn get(key: []const u8) ?[]const u8 {
    const key_slice = RawSliceConst(u8).from(key);
    const result = sysenv_get(key_slice).into() orelse return null;

    return result.into();
}

/// Set the value of an environment variable with key `key` to `value`.
pub fn set(key: []const u8, value: []const u8) void {
    const key_slice = RawSliceConst(u8).from(key);
    const value_slice = RawSliceConst(u8).from(value);
    return sysenv_set(key_slice, value_slice);
}

/// Remove an environment variable with key `key`.
/// does nothing if the variable does not exist.
pub fn remove(key: []const u8) void {
    const key_slice = RawSliceConst(u8).from(key);
    sysenv_remove(key_slice);
}

/// Check if an environment variable with key `key` exists.
pub fn contains(key: []const u8) bool {
    return get(key) != null;
}
