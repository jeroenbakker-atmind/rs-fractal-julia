.set input_cx, 0
.set input_cy, 4
.set input_iteration, 8

.text
.globl julia_sample_xmm

julia_sample_xmm:
    movq input_iteration(%rsi), %rax
    movq %rax, (%rdi)
    ret