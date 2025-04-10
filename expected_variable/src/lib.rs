extern crate case;

extern crate case;

use case::CaseExt;

pub fn expected_variable(original: &str, expected: &str) -> Option<String> {
    if original.trim() != original
        || original.chars().any(|c| c.is_whitespace())
        || (!original.is_camel_case() && !original.is_snake_case())
    {
        return None;
    }

    let original_lower = original.to_lowercase();
    let expected_lower = expected.to_lowercase();

    let diff = edit_distance(&original_lower, &expected_lower);
    let max_len = std::cmp::max(original.len(), expected.len());

    let similarity = ((max_len - diff) * 100) as f64 / max_len as f64;
    let similarity = similarity.ceil();

    (similarity >= 50.0).then(|| format!("{}%", similarity))
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let w1 = source.chars().collect::<Vec<_>>();
    let w2 = target.chars().collect::<Vec<_>>();

    let source_length = w1.len() + 1;
    let target_length = w2.len() + 1;

    let mut matrix = vec![vec![0; source_length]; target_length];

    for i in 1..source_length {
        matrix[0][i] = i;
    }
    for j in 1..target_length {
        matrix[j][0] = j;
    }

    for j in 1..target_length {
        for i in 1..source_length {
            let x: usize = if w1[i - 1] == w2[j - 1] {
                matrix[j - 1][i - 1]
            } else {
                1 + std::cmp::min(
                    std::cmp::min(matrix[j][i - 1], matrix[j - 1][i]),
                    matrix[j - 1][i - 1],
                )
            };
            matrix[j][i] = x;
        }
    }
    matrix[target_length - 1][source_length - 1]
}
