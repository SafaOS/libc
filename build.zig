const std = @import("std");

// Although this function looks imperative, note that its job is to
// declaratively construct a build graph that will be executed by an external
// runner.
pub fn build(b: *std.Build) void {
    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});
    const target = b.standardTargetOptions(.{ .default_target = .{
        .abi = .none,
        .os_tag = .freestanding,
        .cpu_arch = .x86_64,
        .cpu_features_sub = std.Target.x86.featureSet(&[_]std.Target.x86.Feature{ .avx, .avx2, .sse, .sse2, .sse3 }),
        .cpu_features_add = std.Target.x86.featureSet(&[_]std.Target.x86.Feature{.soft_float}),
    } });

    const lib = b.addStaticLibrary(.{
        .name = "salibc",
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = false,
    });
    lib.bundle_compiler_rt = true;

    const lib_check = b.addStaticLibrary(.{
        .name = "salibc",
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    // This declares intent for the library to be installed into the standard
    // location when the user invokes the "install" step (the default step when
    // running `zig build`).
    b.installArtifact(lib);
    const check = b.step("check", "checks if libc compiles");
    check.dependOn(&lib_check.step);
}
