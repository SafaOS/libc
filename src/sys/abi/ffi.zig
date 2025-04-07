pub fn Optional(comptime T: type) type {
    return extern struct {
        const Self = @This();
        tag: enum(u8) {
            None,
            Some,
        },
        value: extern union {
            none: void,
            some: T,
        } = .{ .none = {} },

        pub fn from(value: ?T) Self {
            return switch (value) {
                null => Self{ .tag = .None },
                else => Self{ .tag = .Some, .value = .{ .some = value } },
            };
        }

        pub fn into(self: Self) ?T {
            return switch (self.tag) {
                .None => null,
                .Some => self.value.some,
            };
        }
    };
}

pub fn RawSlice(comptime T: type) type {
    return extern struct {
        const Self = @This();
        ptr: [*]T,
        len: usize,

        pub fn from(slice: []T) Self {
            return Self{ .ptr = slice.ptr, .len = slice.len };
        }

        pub fn into(self: Self) []T {
            return self.ptr[0..self.len];
        }
    };
}

pub const SpawnFlags = packed struct {
    const Self = @This();
    clone_resources: bool = false,
    clone_cwd: bool = false,
    __padding: u6 = 0,
};

pub const InodeType = enum(u8) {
    File,
    Directory,
    Device,
};

pub const FileAttrs = extern struct {
    kind: InodeType,
    size: usize,
};

pub const DirEntry = extern struct {
    const Self = @This();
    attrs: FileAttrs,
    name_len: usize,
    name_raw: [128:0]u8,

    pub fn name(self: Self) []const u8 {
        return self.name_raw[0..self.name_len];
    }
};
