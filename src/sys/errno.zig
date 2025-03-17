const std = @import("std");

pub const SysError = enum(u16) {
    pub const Self = @This();
    None = 0,
    // use when no ErrorStatus is avalible for xyz and you cannot add a new one
    Generic = 1,
    OperationNotSupported = 2,
    // for example an elf class is not supported, there is a difference between NotSupported and
    // OperationNotSupported
    NotSupported = 3,
    // for example a magic value is invaild
    Corrupted = 4,
    InvaildSyscall = 5,
    InvaildResource = 6,
    InvaildPid = 7,
    InvaildOffset = 8,
    // instead of panicking syscalls will return this on null and unaligned pointers
    InvaildPtr = 9,
    // for operations that requires a vaild utf8 str...
    InvaildStr = 0xA,
    // for operations that requires a str that doesn't exceed a max length such as
    // file names (128 bytes)
    StrTooLong = 0xB,
    InvaildPath = 0xC,
    NoSuchAFileOrDirectory = 0xD,
    NotAFile = 0xE,
    NotADirectory = 0xF,
    AlreadyExists = 0x10,
    NotExecutable = 0x11,
    // would be useful when i add remove related operations to the vfs
    DirectoryNotEmpty = 0x12,
    // Generic premissions(protection) related error
    MissingPermissions = 0x13,
    // memory allocations and mapping error, most likely that memory is full
    MMapError = 0x14,
    Busy = 0x15,
    // errors sent by processes
    NotEnoughArguments = 0x16,
    OutOfMemory = 0x17,
    Last = 0x18,
    // iso
    StreamTooLong,
    StreamTooShort,
    EndOfStream,
    ArgumentOutOfDomain,
    IllegalByteSequence,

    pub fn into_err(self: Self) Error!void {
        switch (self) {
            .None => return,
            .Last => unreachable,
            inline else => |x| {
                const src_tag = @tagName(x);
                return @field(Error, src_tag);
            },
        }
    }

    pub fn from_err(other: anytype) ?Self {
        switch (other) {
            inline else => |x| {
                const src_tag = @errorName(x);
                if (@hasField(Self, src_tag)) {
                    return @field(Self, src_tag);
                } else {
                    return null;
                }
            },
        }
    }

    pub fn from_u16(value: u16) ?Self {
        if (value < @intFromEnum(Self.Last)) {
            return @enumFromInt(value);
        } else {
            return null;
        }
    }
};

pub const Error = error{
    // use when no ErrorStatus is avalible for xyz and you cannot add a new one
    Generic,
    OperationNotSupported,
    // for example an elf class is not supported, there is a difference between NotSupported and
    // OperationNotSupported
    NotSupported,
    // for example a magic value is invaild
    Corrupted,
    InvaildSyscall,
    InvaildResource,
    InvaildPid,
    InvaildOffset,
    // instead of panicking syscalls will return this on null and unaligned pointers
    InvaildPtr,
    // for operations that requires a vaild utf8 str...
    InvaildStr,
    StrTooLong,
    InvaildPath,
    NoSuchAFileOrDirectory,
    NotAFile,
    NotADirectory,
    AlreadyExists,
    NotExecutable,
    // would be useful when i add remove related operations to the vfs
    DirectoryNotEmpty,
    // Generic premissions(protection) related error
    MissingPermissions,
    // memory allocations and mapping error, most likely that memory is full
    OutOfMemory,
    Busy,
    NotEnoughArguments,
    // iso
    StreamTooLong,
    StreamTooShort,
    EndOfStream,
    ArgumentOutOfDomain,
    IllegalByteSequence,
    MMapError,
    Unknown,
};

pub export var errno: u32 = 0;
pub fn geterr() ?Error {
    const err: u16 = @truncate(errno);
    const syserr = SysError.from_u16(err);
    const errset = if (syserr) |non_null| non_null.into_err() else Error.Unknown;

    errset catch |e| return e;
    return null;
}

pub fn seterr(err: Error) void {
    const syserror = SysError.from_err(err);
    if (syserror) |non_null|
        errno = @intFromEnum(non_null)
    else
        errno = @intFromError(err);
}

export fn __errno_location() *u32 {
    return &errno;
}
