// a value form 0 to 255 4 bytes
fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow")
}

// from -32768 to 32768
fn diff(a: i16, b:i16) -> i16 {
    a.checked_sub(b).expect("'ERROR: attempt to subtract with overflow'")
}

// values betweeen -128 and 127
fn pro(a: i8, b:i8) -> i8 {
    a.checked_mul(b).expect("'ERROR: attempt to multiply with overflow'")
}


fn quo(a: i32, b:i32) -> i32 {
    if b == 0 {
        panic!("'ERROR: attempt to add with overflow'");
    }
    a.checked_div(b).expect("'ERROR: attempt to divide with overflow'")
}

fn rem(a: i32, b:i32) -> i32 {
    if b == 0 {
        panic!("'ERROR: attempt to add with overflow'");
    }
    a.checked_rem(b).expect("'ERROR: attempt to add with overflow'")
}