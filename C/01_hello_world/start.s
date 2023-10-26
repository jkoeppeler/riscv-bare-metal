.section .init
.globl _start
_start:
    lla sp, (_stack)
    j main
