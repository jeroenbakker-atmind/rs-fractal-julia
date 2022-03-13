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
.set input_scratch4, 52
.set input_scratch5, 56
.set input_scratch6, 60
.set input_scratch7, 64

.text
.globl julia_sample_ymm_f32_packed

# Register usage:
# ymm0 is the zx value
# ymm1 is the zy value
# ymm2 contains the r2 value.
# ymm3 contains the cy
# ymm4 contains the cx

# inner loop (per iteration)
# ymm5 = zx * zx
# ymm6 = zy * zy
# ymm7 = temp value for calculations.
# ymm8 is the adder for zx
# ymm9 contains the init value for cx.


julia_sample_ymm_f32_packed:

    # xmm8 = (zx_max - zx_min) / width
    mov rcx, [rsi+input_width]

    movss xmm8, [rsi+input_zx_max]
    subss xmm8, [rsi+input_zx_min]
    vcvtsi2ss xmm9, xmm9, rcx
    divss xmm8, xmm9

    # ecx contains the width. Due to vectorization we can do 8 at a time
    # so it is divided by 8.
    shr ecx, 3


    # ymm9 = zx
    movss xmm9, [rsi+input_zx_min]
    movss [rsi+input_scratch0], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch1], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch2], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch3], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch4], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch5], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch6], xmm9
    addss xmm9, xmm8
    movss [rsi+input_scratch7], xmm9
    vmovups ymm9, [rsi+input_scratch]

    # ymm8 is the adder for the ymm9 xmm8 is multiplied by 8 as 8 samples are calculated at the same time.
    addss xmm8, xmm8
    addss xmm8, xmm8
    addss xmm8, xmm8
    vbroadcastss ymm8, xmm8


    movss xmm2, [rsi+input_r2]
    vbroadcastss ymm2, xmm2
    movss xmm3, [rsi+input_cy]
    vbroadcastss ymm3, xmm3
    movss xmm4, [rsi+input_cx]
    vbroadcastss ymm4, xmm4
    mov edx, [rsi+input_max_iteration]

_julia_next_sample:
    # reset iteration
    vpxor ymm10, ymm10, ymm10
    xor eax, eax

    # reset zx.
    vmovaps ymm0, ymm9
    movss xmm1, [rsi+input_zy]
    vbroadcastss ymm1, xmm1

    # reset the k0 mask.
    # use this mask for the main instructions. update the k0 mask with the r2 check.

_julia_next_iteration:
    # ymm5 = zx * zx
    vmulps ymm5, ymm0, ymm0
    # ymm6 = zy * zy
    vmulps ymm6, ymm1, ymm1

    # ymm7 = (zx * zx) + (zy * zy)
    vaddps ymm7, ymm5, ymm6
    # exit when larger or equal to r2
    vcmpltps ymm11, ymm7, ymm2

    vptest ymm11, ymm11
    jz _julia_store_and_return

    # exit when iteration same as max iteration.
    cmp eax, edx
    jnl _julia_store_and_return

    # ymm7 = (zx*zx) - (zy*zy)
    vsubps ymm7, ymm5, ymm6
    # zx = 2 * zx * zy + cy
    vaddps ymm1, ymm1, ymm1
    vmulps ymm1, ymm1, ymm0
    vaddps ymm1, ymm1, ymm3

    # zx = xmm7 + cx
    vaddps ymm0, ymm7, ymm4

    inc eax
    movd xmm7, eax
    vbroadcastss ymm7, xmm7
    vblendvps ymm10, ymm10, ymm7, ymm11
    jmp _julia_next_iteration

_julia_store_and_return:
    vmovups [rdi], ymm10

    add rdi, 4 * 8

    # add delta per sample to zx
    vaddps ymm9, ymm9, ymm8

    dec ecx
    jnz _julia_next_sample

    ret

