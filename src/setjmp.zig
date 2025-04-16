const jmp_buf_t = packed struct {
    rbx: usize,
    rbp: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
    rsp: usize,
    rip: usize,
};

export fn setjmp(buf: *jmp_buf_t) callconv(.naked) void {
    _ = buf;
    asm volatile (
        \\ movq %rbx, 0x0(%rdi)
        \\ movq %rbp, 0x8(%rdi)
        \\ movq %r12, 0x10(%rdi)
        \\ movq %r13, 0x18(%rdi)
        \\ movq %r14, 0x20(%rdi)
        \\ movq %r15, 0x28(%rdi)
        \\ lea 0x8(%rsp), %rax
        \\ movq %rax, 0x30(%rdi)
        \\ movq (%rsp), %rax
        \\ mov %rax, 0x38(%rdi)
        \\ xor %rax, %rax
        \\ ret
    );
}

export fn longjmp(buf: *jmp_buf_t, val: c_int) callconv(.naked) noreturn {
    _ = buf;
    _ = val;
    asm volatile (
    // set val to 1 if it is 0 and move it to the return register
        \\ movl $1, %eax
        \\ testl %esi, %esi
        \\ cmovnzl %esi, %eax
        // restore
        \\ movq 0x0(%rdi), %rbx
        \\ movq 0x8(%rdi), %rbp
        \\ movq 0x10(%rdi), %r12
        \\ movq 0x18(%rdi), %r13
        \\ movq 0x20(%rdi), %r14
        \\ movq 0x28(%rdi), %r15
        \\ movq 0x30(%rdi), %rsp
        \\ movq 0x38(%rdi), %rcx
        \\ jmp *%rcx
    );
}
