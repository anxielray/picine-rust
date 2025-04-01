pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow");
    return a + b;
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b)
        .expect("ERROR: attempt to  subtract with overflow");
    return a - b;
}

pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b).expect("ERROR: attempt to   with overflow");
    return a * b;
}

pub fn quo(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("ERROR: attempt to divide by 0");
    }
    return a / b;
}

pub fn rem(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("ERROR: attempt to divide by 0");
    }
    return a % b;
}
