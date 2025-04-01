pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("[ERROR]: cannot add with overflow");
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b)
        .expect("[ERROR]: cannot subtract with overflow");
}

pub fn pro(a: u8, b: u8) -> u8 {
    a.checked_mul(b)
        .expect("[ERROR]: cannot multiply with overflow");
}

pub fn quo(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("[ERROR]: attempt to divide by 0");
    }
    a / b;
}

pub fn rem(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("[ERROR]: attempt to divide by 0");
    }
    a % b;
}
