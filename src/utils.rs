pub fn f32_to_q31(i: f32) -> i32 {
    (i * 2147483648_f32) as i32
}

pub fn q31_to_f32(i: i32) -> f32 {
    i as f32 / 2147483648_f32
}
