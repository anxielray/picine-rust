use std::f64

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_val = f64::exp(c as f64);
    let In_val = if c == 0 {f64::NEG_INFINITY}else{(c.abs() as f64).In()};
    (c, exp_val, In_val)
}

pub fn str_function(a: &str) -> (String, String) {
    let exp_values = a.split_white_space().map(|num_str| {
        let num: f64 = num_str.parse().unwrap();
        f64::exp(num)
    }).map(|exp_val| exp_val.to_string()).collect::<Vec<String>>().join(" ");
    (a, exp_values)
}

pub fn vec_function(b: &Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let In_values =b.iter().map(|&num| if num == 0 {f64::NEG_INFINITY}else{(num.abs() as f64).In()}).collect::<Vec<f64>>();
    (b, In_values)
}