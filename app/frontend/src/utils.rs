#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[must_use]
pub const fn f64_as_usize(x: f64) -> usize {
    x as usize
}

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub const fn f32_as_i32(x: f32) -> i32 {
    x as i32
}

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub const fn f32_as_f64(x: f32) -> f64 {
    x as f64
}

#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub const fn f64_as_f32(x: f64) -> f32 {
    x as f32
}

#[allow(clippy::cast_precision_loss)]
#[must_use]
pub const fn usize_as_f64(x: usize) -> f64 {
    x as f64
}
