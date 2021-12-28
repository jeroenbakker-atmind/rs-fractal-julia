.set input_zy, 0
.set input_r2, 8
.set input_cx, 16
.set input_cy, 24
.set input_zx_min, 32
.set input_zx_max, 40
.set input_width, 48
.set input_max_iteration, 56

.text
.globl julia_sample_xmm_f64_scalar

julia_sample_xmm_f64_scalar:
    mov ecx, [rsi+input_width]
    # xmm8 = (zx_max - zx_min) / width
    movsd xmm8, [rsi+input_zx_max]
    subsd xmm8, [rsi+input_zx_min]
    cvtsi2sd xmm9, rcx
    divsd xmm8, xmm9

    # xmm9 = zx
    movsd xmm9, [rsi+input_zx_min]
    movsd xmm2, [rsi+input_r2]
    movsd xmm3, [rsi+input_cy]
    movsd xmm4, [rsi+input_cx]
    mov edx, [rsi+input_max_iteration]

_julia_next_sample:
    # reset iteration
    xor eax, eax
    # reset zx.
    movsd xmm0, xmm9
    movsd xmm1, [rsi+input_zy]


_julia_next_iteration:
    # xmm5 = zx * zx
    vmulsd xmm5, xmm0, xmm0
    # xmm6 = zy * zy
    vmulsd xmm6, xmm1, xmm1

    # xmm7 = (zx * zx) + (zy * zy)
    vaddsd xmm7, xmm5, xmm6
    # exit when larger or equal to r2
    comisd xmm7, xmm2
    jnb _julia_store_and_return

    # exit when iteration same as max iteration.
    cmp eax, edx
    jnl _julia_store_and_return

    # xmm7 = (zx*zx) - (zy*zy)
    vsubsd xmm7, xmm5, xmm6
    # zx = 2 * zx * zy + cy
    addsd xmm1, xmm1
    mulsd xmm1, xmm0
    addsd xmm1, xmm3

    # zx = xmm7 + cx
    vaddsd xmm0, xmm7, xmm4

    inc eax
    jmp _julia_next_iteration

_julia_store_and_return:
    #mov rax, [rsi+input_iteration]
    mov [rdi], rax

    add rdi, 8

    # add delta per sample to zx
    addsd xmm9, xmm8
    dec ecx
    jnz _julia_next_sample
    ret
