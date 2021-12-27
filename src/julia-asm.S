.set input_zy, 0
.set input_r2, 4
.set input_cx, 8
.set input_cy, 12
.set input_zx_min, 16
.set input_zx_max, 20
.set input_width, 24
.set input_max_iteration, 32

.text
.globl julia_row_xmm_scalar

julia_row_xmm_scalar:
    mov ecx, [rsi+input_width]
    # xmm8 = (zx_max - zx_min) / width
    movss xmm8, [rsi+input_zx_max]
    subss xmm8, [rsi+input_zx_min]
    cvtsi2ss xmm9, rcx
    divss xmm8, xmm9

    # xmm9 = zx
    movss xmm9, [rsi+input_zx_min]
    movss xmm2, [rsi+input_r2]
    movss xmm3, [rsi+input_cy]
    movss xmm4, [rsi+input_cx]
    mov edx, [rsi+input_max_iteration]

_julia_next_sample:
    # reset iteration
    xor eax, eax
    # reset zx.
    movss xmm0, xmm9
    movss xmm1, [rsi+input_zy]


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

    add rdi, 8

    # add delta per sample to zx
    addss xmm9, xmm8
    dec ecx
    jnz _julia_next_sample
    ret
