use core::ffi::c_int;

#[cfg(target_arch = "aarch64")]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct JmpBufT {
    x19: usize,
    x20: usize,
    x21: usize,
    x22: usize,
    x23: usize,
    x24: usize,
    x25: usize,
    x26: usize,
    x27: usize,
    x28: usize,
    x29: usize,
    x30: usize,
    sp: usize,
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct JmpBufT {
    rbx: usize,
    rbp: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
    rsp: usize,
    rip: usize,
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn setjmp(buf: *const JmpBufT) -> c_int {
    #[cfg(target_arch = "aarch64")]
    core::arch::naked_asm!(
        "
            stp  x19, x20, [x0,#0]
            stp  x21, x22, [x0,#16]
            stp  x23, x24, [x0,#32]
            stp  x25, x26, [x0,#48]
            stp  x27, x28, [x0,#64]
            stp  x29, x30, [x0,#80]
            mov  x2, sp
            str  x2, [x0, #96]
            mov  x0, #0
            ret
        "
    );
    #[cfg(target_arch = "x86_64")]
    core::arch::naked_asm!(
        "
        movq %rbx, 0x0(%rdi)
        movq %rbp, 0x8(%rdi)
        movq %r12, 0x10(%rdi)
        movq %r13, 0x18(%rdi)
        movq %r14, 0x20(%rdi)
        movq %r15, 0x28(%rdi)
        leaq 0x8(%rsp), %rax
        movq %rax, 0x30(%rdi)
        movq (%rsp), %rax
        movq %rax, 0x38(%rdi)
        xorq %rax, %rax
        ret
        ",
        options(att_syntax)
    );
}
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn longjmp(buf: *const JmpBufT, val: c_int) -> ! {
    #[cfg(target_arch = "aarch64")]
    core::arch::naked_asm!(
        "
        ldp  x19, x20, [x0,#0]
        ldp  x21, x22, [x0,#16]
        ldp  x23, x24, [x0,#32]
        ldp  x25, x26, [x0,#48]
        ldp  x27, x28, [x0,#64]
        ldp  x29, x30, [x0,#80]
        ldr  x2, [x0,#96]
        mov  sp, x2
        /* Move the return value in place, but return 1 if passed 0. */
        adds x0, xzr, x1
        csinc x0, x0, xzr, ne
        ret
        "
    );
    #[cfg(target_arch = "x86_64")]
    core::arch::naked_asm!(
        "
       // set val to 1 if it is 0 and move it to the return register
       xor %rax, %rax
       movq $1, %rax
       testl %esi, %esi
       cmovnzl %esi, %eax
       // restore
       movq 0x0(%rdi), %rbx
       movq 0x8(%rdi), %rbp
       movq 0x10(%rdi), %r12
       movq 0x18(%rdi), %r13
       movq 0x20(%rdi), %r14
       movq 0x28(%rdi), %r15
       movq 0x30(%rdi), %rsp
       movq 0x38(%rdi), %rsi
       jmp *%rsi
        ",
        options(att_syntax)
    );
}
