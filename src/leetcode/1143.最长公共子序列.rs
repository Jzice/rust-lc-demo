/*!
 * # [1143.最长公共子序列](https://leetcode.cn/problems/longest-common-subsequence/description/)
 *
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * ## 难度
 *
 * - Medium (64.96%)
 * - Likes:    1215
 * - Dislikes: 0
 * - Total Accepted:    306.5K
 * - Total Submissions: 471.8K
 * - Testcase Example:  '"abcde"\n"ace"'
 *
 * ## 问题描述
 *
 * 给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。
 *
 * 一个字符串的**子序列**是指这样一个新的字符串：
 *
 * 它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
 *
 * 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。
 *
 * 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
 *
 * ## 示例 1：
 * - 输入：text1 = "abcde", text2 = "ace"
 * - 输出：3
 * - 解释：最长公共子序列是 "ace" ，它的长度为 3 。
 *
 * ## 示例 2：
 * - 输入：text1 = "abc", text2 = "abc"
 * - 输出：3
 * - 解释：最长公共子序列是 "abc" ，它的长度为 3 。
 *
 * ## 示例 3：
 * - 输入：text1 = "abc", text2 = "def"
 * - 输出：0
 * - 解释：两个字符串没有公共子序列，返回 0 。
 *
 * ## 提示：
 * - text1 和 text2 仅由小写英文字符组成。
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 最长公共子序列
    /// - 动态规划
    /// 1. 设 dp[i][j]: 以text1[0..i], text2[0..j]的最长公共子序列
    /// 2. 初始条件：
    ///     dp[0][j] = 0
    ///     dp[i][0] = 0
    /// 3. 递推关系:
    ///    dp[i][j] = dp[i-1][j-1] + 1,           (text1[i] == text2[j])
    ///        或者 = max(dp[i-1][j], dp[i][j-1]),(text1[i] != text2[j])
    /// 4. 目标值：dp[l1][l2]
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                dp[i + 1][j + 1] = if c1 == c2 {
                    dp[i][j] + 1
                } else {
                    std::cmp::max(dp[i][j + 1], dp[i + 1][j])
                }
            }
        }
        dp[text1.len()][text2.len()] as i32
    }
}
// @lc code=end
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".into(), "ace".into()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".into(), "abc".into()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".into(), "def".into()),
            0
        );
    }
}
