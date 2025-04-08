pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();
    let source = source.chars().collect::<Vec<_>>();
    let target = target.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] =
                    1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i][j - 1], dp[i - 1][j]));
            }
        }
    }

    dp[m][n]
}
