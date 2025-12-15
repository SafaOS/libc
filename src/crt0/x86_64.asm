extern _salibc_start

_start:
    and rsp, ~0xf
    push rbp
    push rbp
    call _salibc_start
    ud2
