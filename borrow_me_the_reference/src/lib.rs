pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut skip_next = false;

    for c in s.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match c {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            }
            '+' => {
                skip_next = true;
            }
            _ => result.push(c),
        }
    }
    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for eq in v.iter_mut() {
        if let Some(operator_pos) = eq.find(|c| c == '+' || c == '-') {
            let (left, right) = eq.split_at(operator_pos);
            let operator = eq.chars().nth(operator_pos).unwrap();
            let right = &right[1..];
            let left_num: i32 = left.parse().unwrap();
            let right_num: i32 = right.parse().unwrap();
            let result = match operator {
                '+' => left_num + right_num,
                '-' => left_num - right_num,
                _ => unreachable!(),
            };
            *eq = result.to_string();
        }
    }
}
