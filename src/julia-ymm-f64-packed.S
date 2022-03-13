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
.set input_scratch2, 80
.set input_scratch3, 88

.text
.globl julia_sample_ymm_f64_packed

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


julia_sample_ymm_f64_packed:

    # xmm8 = (zx_max - zx_min) / width
    mov rcx, [rsi+input_width]

    movsd xmm8, [rsi+input_zx_max]
    subsd xmm8, [rsi+input_zx_min]
    vcvtsi2sd xmm9, xmm9, rcx
    divsd xmm8, xmm9

    # ecx contains the width. Due to vectorization we can do 4 at a time
    # so it is divided by 4.
    shr ecx, 2

    # ymm9 = zx
    movsd xmm9, [rsi+input_zx_min]
    movsd [rsi+input_scratch0], xmm9
    addsd xmm9, xmm8
    movsd [rsi+input_scratch1], xmm9
    addsd xmm9, xmm8
    movsd [rsi+input_scratch2], xmm9
    addsd xmm9, xmm8
    movsd [rsi+input_scratch3], xmm9
    vmovupd ymm9, [rsi+input_scratch]

    # ymm8 is the adder for the ymm9 ymm8 is multiplied by 4 as 4 samples are calculated at the same time.
    addsd xmm8, xmm8
    addsd xmm8, xmm8
    vbroadcastsd ymm8, xmm8

    movsd xmm2, [rsi+input_r2]
    vbroadcastsd ymm2, xmm2
    movsd xmm3, [rsi+input_cy]
    vbroadcastsd ymm3, xmm3
    movsd xmm4, [rsi+input_cx]
    vbroadcastsd ymm4, xmm4
    mov edx, [rsi+input_max_iteration]

_julia_next_sample:
    # reset iteration
    vpxor ymm10, ymm10, ymm10
    xor eax, eax

    # reset zx.
    vmovapd ymm0, ymm9
    movsd xmm1, [rsi+input_zy]
    vbroadcastsd ymm1, xmm1

    # reset the k0 mask.
    # use this mask for the main instructions. update the k0 mask with the r2 check.

_julia_next_iteration:
    # ymm5 = zx * zx
    vmulpd ymm5, ymm0, ymm0
    # ymm6 = zy * zy
    vmulpd ymm6, ymm1, ymm1

    # ymm7 = (zx * zx) + (zy * zy)
    vaddpd ymm7, ymm5, ymm6
    # exit when larger or equal to r2
    vcmpltpd ymm11, ymm7, ymm2

    vptest ymm11, ymm11
    jz _julia_store_and_return

    # exit when iteration same as max iteration.
    cmp eax, edx
    jnl _julia_store_and_return

    # ymm7 = (zx*zx) - (zy*zy)
    vsubpd ymm7, ymm5, ymm6
    # zx = 2 * zx * zy + cy
    vaddpd ymm1, ymm1, ymm1
    vmulpd ymm1, ymm1, ymm0
    vaddpd ymm1, ymm1, ymm3

    # zx = ymm7 + cx
    vaddpd ymm0, ymm7, ymm4

    inc eax
    movq [rsi+input_scratch0], rax
    movq xmm7, [rsi+input_scratch0]
    # This is a packed double move, but we wanted an quad word move.
    vbroadcastsd ymm7, xmm7
    vblendvpd ymm10, ymm10, ymm7, ymm11
    jmp _julia_next_iteration

_julia_store_and_return:
    vmovupd [rsi+input_scratch0], ymm10
    movq rax, [rsi+input_scratch0]
    mov [rdi], eax
    movq rax, [rsi+input_scratch1]
    mov [rdi+4], eax
    movq rax, [rsi+input_scratch2]
    mov [rdi+8], eax
    movq rax, [rsi+input_scratch3]
    mov [rdi+12], eax

    add rdi, 4 * 4

    # add delta per sample to zx
    vaddpd ymm9, ymm9, ymm8

    dec ecx
    jnz _julia_next_sample

    ret

