pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut skip_count = 0;

    while let Some(c) = chars.next() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match c {
            '-' => {
                result.pop(); // Remove the last character for backspace
            }
            '+' => {
                // Count consecutive '+' signs
                skip_count += 1;
                while let Some(next_char) = chars.peek() {
                    if *next_char == '+' {
                        skip_count += 1;
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Skip characters equal to the number of consecutive '+' signs
            }
            _ => {
                result.push(c); // Add current character to result
            }
        }
    }

    *s = result;
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
