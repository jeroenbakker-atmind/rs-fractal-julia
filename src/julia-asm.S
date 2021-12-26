.set input_zx, 0
.set input_zy, 4
.set input_r2, 8
.set input_cx, 12
.set input_cy, 16
.set input_max_iteration, 24

.text
.globl julia_sample_xmm_scalar

julia_sample_xmm_scalar:
    xor eax, eax
    movss xmm0, [rsi+input_zx]
    movss xmm1, [rsi+input_zy]
    movss xmm2, [rsi+input_r2]
    movss xmm3, [rsi+input_cy]
    movss xmm4, [rsi+input_cx]
    mov edx, [rsi+input_max_iteration]

_julia_next_iteration:
    # xmm5 = zx * zx
    vmulss xmm5, xmm0, xmm0
    # xmm6 = zy * zy
    vmulss xmm6, xmm1, xmm1

    # xmm7 = (zx * zx) + (zy * zy)
    vaddss xmm7, xmm5, xmm6
    # exit when larger or equal to r2
    comiss xmm7, xmm2
    jnb _julia_store_and_return

    # exit when iteration same as max iteration.
    cmp eax, edx
    jnl _julia_store_and_return

    # xmm7 = (zx*zx) - (zy*zy)
    vsubss xmm7, xmm5, xmm6
    # zx = 2 * zx * zy + cy
    addss xmm1, xmm1
    mulss xmm1, xmm0
    addss xmm1, xmm3

    # zx = xmm7 + cx
    vaddss xmm0, xmm7, xmm4

    inc eax
    jmp _julia_next_iteration

_julia_store_and_return:
    #mov rax, [rsi+input_iteration]
    mov [rdi], rax
    ret
