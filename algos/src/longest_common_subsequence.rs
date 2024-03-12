#[allow(dead_code)]
pub fn longest_common_subbsequence(text1: String, text2: String) -> i32{
    let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();

        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for j in (0..text2.len()).rev() {
            for i in (0..text1.len()).rev() {
                if text1[i] != text2[j] {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
                } else {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                }
            }
        }
        dp[0][0]
    
}


#[cfg(test)]
mod lcs_tests {
    use super::*;

    #[test]
    fn lcs_base() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();

        assert_eq!(longest_common_subbsequence(text1, text2), 3);
    }
}