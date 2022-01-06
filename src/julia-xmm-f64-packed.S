.set input_zy, 0
.set input_r2, 8
.set input_cx, 16
.set input_cy, 24
.set input_zx_min, 32
.set input_zx_max, 40
.set input_width, 48
.set input_max_iteration, 56
.set input_scratch, 64
.set input_scratch0, 64
.set input_scratch1, 72
#.set input_scratch2, 80
#.set input_scratch3, 88

.text
.globl julia_sample_xmm_f64_packed

# Register usage:
# xmm0 is the zx value
# xmm1 is the zy value
# xmm2 contains the r2 value.
# xmm3 contains the cy
# xmm4 contains the cx

# inner loop (per iteration)
# xmm5 = zx * zx
# xmm6 = zy * zy
# xmm7 = temp value for calculations.
# xmm8 is the adder for zx
# xmm9 contains the init value for cx.


julia_sample_xmm_f64_packed:

    # xmm8 = (zx_max - zx_min) / width
    mov rcx, [rsi+input_width]

    movsd xmm8, [rsi+input_zx_max]
    subsd xmm8, [rsi+input_zx_min]
    vcvtsi2sd xmm9, xmm9, rcx
    divsd xmm8, xmm9

    # ecx contains the width. Due to vectorization we can do 2 at a time
    # so it is divided by 2.
    shr ecx, 1


    # xmm9 = zx
    movsd xmm9, [rsi+input_zx_min]
    movsd [rsi+input_scratch0], xmm9
    addsd xmm9, xmm8
    movsd [rsi+input_scratch1], xmm9
    movupd xmm9, [rsi+input_scratch]

    # xmm8 is the adder for the xmm9 xmm8 is multiplied by 2 as 2 samples are calculated at the same time.
    addsd xmm8, xmm8
    # CHECK if vmovddup can be used.
    movsd [rsi+input_scratch], xmm8
    movhpd xmm8, [rsi+input_scratch]

    movsd xmm2, [rsi+input_r2]
    movhpd xmm2, [rsi+input_r2]
    movsd xmm3, [rsi+input_cy]
    movhpd xmm3, [rsi+input_cy]
    movsd xmm4, [rsi+input_cx]
    movhpd xmm4, [rsi+input_cx]
    mov edx, [rsi+input_max_iteration]

_julia_next_sample:
    # reset iteration
    pxor xmm10, xmm10
    xor eax, eax

    # reset zx.
    movapd xmm0, xmm9
    movsd xmm1, [rsi+input_zy]
    movhpd xmm1, [rsi+input_zy]

    # reset the k0 mask.
    # use this mask for the main instructions. update the k0 mask with the r2 check.

_julia_next_iteration:
    # xmm5 = zx * zx
    vmulpd xmm5, xmm0, xmm0
    # xmm6 = zy * zy
    vmulpd xmm6, xmm1, xmm1

    # xmm7 = (zx * zx) + (zy * zy)
    vaddpd xmm7, xmm5, xmm6
    # exit when larger or equal to r2
    vcmpltpd xmm11, xmm7, xmm2

    vptest xmm11, xmm11
    jz _julia_store_and_return

    # exit when iteration same as max iteration.
    cmp eax, edx
    jnl _julia_store_and_return

    # xmm7 = (zx*zx) - (zy*zy)
    vsubpd xmm7, xmm5, xmm6
    # zx = 2 * zx * zy + cy
    addpd xmm1, xmm1
    mulpd xmm1, xmm0
    addpd xmm1, xmm3

    # zx = xmm7 + cx
    vaddpd xmm0, xmm7, xmm4

    inc eax
    movq [rsi+input_scratch0], rax
    movq xmm7, [rsi+input_scratch0]
    # This is a packed double move, but we wanted an quad word move.
    movhpd xmm7, [rsi+input_scratch0]
    vblendvpd xmm10, xmm10, xmm7, xmm11
    jmp _julia_next_iteration

_julia_store_and_return:
    movupd [rsi+input_scratch0], xmm10
    movq rax, [rsi+input_scratch0]
    mov [rdi], eax
    movq rax, [rsi+input_scratch1]
    mov [rdi+4], eax


    add rdi, 4 * 2

    # add delta per sample to zx
    addpd xmm9, xmm8

    dec ecx
    jnz _julia_next_sample

    ret

