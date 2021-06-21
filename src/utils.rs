pub fn f32_to_q31(input: f32) -> i32 {
    asm!("VCVT.S32.F32")
}
