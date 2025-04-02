pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = if c == 0 {
        f64::NEG_INFINITY
    } else {
        (c.abs() as f64).ln()
    };
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a.split_whitespace()
        .map(|s| s.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values = b.iter()
        .map(|&n| if n == 0 {
            f64::NEG_INFINITY
        } else {
            (n.abs() as f64).ln()
        })
        .collect();
    (b, ln_values)
}
