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
                    result.pop(); // Remove the last character for backspace
                }
            }
            '+' => {
                skip_next = true; // Skip the next character for delete
            }
            _ => result.push(c),
        }
    }
    *s = result.into_iter().collect(); // Convert Vec<char> back to String
}

pub fn do_operations(v: &mut [String]) {
    for eq in v.iter_mut() {
        if let Some(operator_pos) = eq.find(|c| c == '+' || c == '-') {
            let (left, right) = eq.split_at(operator_pos);
            let operator = eq.chars().nth(operator_pos).unwrap();
            let right = &right[1..]; // Skip the operator itself
            let left_num: i32 = left.parse().unwrap();
            let right_num: i32 = right.parse().unwrap();
            let result = match operator {
                '+' => left_num + right_num,
                '-' => left_num - right_num,
                _ => unreachable!(),
            };
            *eq = result.to_string(); // Replace equation with result
        }
    }
}
