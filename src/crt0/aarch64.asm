.global _start

_start:
    mov fp, #0
    sub sp, sp, #16
    stp xzr, xzr, [sp]
    bl _salibc_start
