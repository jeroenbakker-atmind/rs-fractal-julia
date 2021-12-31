.set input_zy, 0
.set input_r2, 4
.set input_cx, 8
.set input_cy, 12
.set input_zx_min, 16
.set input_zx_max, 20
.set input_width, 24
.set input_max_iteration, 32
.set input_scratch, 36
.set input_scratch0, 36
.set input_scratch1, 40
.set input_scratch2, 44
.set input_scratch3, 48

.text
.globl julia_sample_xmm_f32_packed

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


julia_sample_xmm_f32_packed:

    # xmm8 = (zx_max - zx_min) / width
    mov rcx, [rsi+input_width]

    movss xmm8, [rsi+input_zx_max]
    subss xmm8, [rsi+input_zx_min]
    vcvtsi2ss xmm9, xmm9, rcx
    divss xmm8, xmm9

    # ecx contains the width. Due to vectorization we can do 4 at a time
    # so it is divided by 4.
    shr ecx, 2


    # xmm9 = zx
    movss xmm9, [rsi+input_zx_min]
    movss [rsi+input_scratch0], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch1], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch2], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch3], xmm9
    movups xmm9, [rsi+input_scratch]

    # xmm8 is the adder for the xmm9 xmm8 is multiplied by 4 as 4 samples are calculated at the same time.
    addss xmm8, xmm8
    addss xmm8, xmm8
    vbroadcastss xmm8, xmm8


    movss xmm2, [rsi+input_r2]
    vbroadcastss xmm2, xmm2
    movss xmm3, [rsi+input_cy]
    vbroadcastss xmm3, xmm3
    movss xmm4, [rsi+input_cx]
    vbroadcastss xmm4, xmm4
    mov edx, [rsi+input_max_iteration]

_julia_next_sample:
    # reset iteration
    pxor xmm10, xmm10
    xor eax, eax

    # reset zx.
    movaps xmm0, xmm9
    movss xmm1, [rsi+input_zy]
    vbroadcastss xmm1, xmm1

    # reset the k0 mask.
    # use this mask for the main instructions. update the k0 mask with the r2 check.

_julia_next_iteration:
    # xmm5 = zx * zx
    vmulps xmm5, xmm0, xmm0
    # xmm6 = zy * zy
    vmulps xmm6, xmm1, xmm1

    # xmm7 = (zx * zx) + (zy * zy)
    vaddps xmm7, xmm5, xmm6
    # exit when larger or equal to r2
    vcmpltps xmm11, xmm7, xmm2

    vptest xmm11, xmm11
    jz _julia_store_and_return

    # exit when iteration same as max iteration.
    cmp eax, edx
    jnl _julia_store_and_return

    # xmm7 = (zx*zx) - (zy*zy)
    vsubps xmm7, xmm5, xmm6
    # zx = 2 * zx * zy + cy
    addps xmm1, xmm1
    mulps xmm1, xmm0
    addps xmm1, xmm3

    # zx = xmm7 + cx
    vaddps xmm0, xmm7, xmm4

    inc eax
    movd xmm7, eax
    vbroadcastss xmm7, xmm7
    vblendvps xmm10, xmm10, xmm7, xmm11
    jmp _julia_next_iteration

_julia_store_and_return:
    movups [rdi], xmm10

    add rdi, 4 * 4

    # add delta per sample to zx
    addps xmm9, xmm8

    dec ecx
    jnz _julia_next_sample

    ret

