/*!
 * # [1433.检查一个字符串是否可以打破另一个字符串](https://leetcode.cn/problems/check-if-a-string-can-break-another-string/description/)
 *
 * @lc app=leetcode.cn id=1433 lang=rust
 *
 * ## 难度
 *
 * - Medium (64.13%)
 * - Likes:    24
 * - Dislikes: 0
 * - Total Accepted:    6.6K
 * - Total Submissions: 10.3K
 * - Testcase Example:  '"abc"\n"xya"'
 *
 * ## 问题描述
 *
 * 给你两个字符串 s1 和 s2 ，它们长度相等，请你检查是否存在一个 s1  的排列可以打破 s2 的一个排列，或者是否存在一个 s2 的排列可以打破
 * s1 的一个排列。
 * 
 * 字符串 x 可以打破字符串 y （两者长度都为 n ）需满足对于所有 i（在 0 到 n - 1 之间）都有 x[i] >=
 * y[i]（字典序意义下的顺序）。
 * 
 * 
 * ## 示例 1：
 * 
 * - 输入：s1 = "abc", s2 = "xya"
 * - 输出：true
 * - 解释："ayx" 是 s2="xya" 的一个排列，"abc" 是字符串 s1="abc" 的一个排列，且 "ayx" 可以打破 "abc" 。
 * 
 * 
 * ## 示例 2：
 * 
 * - 输入：s1 = "abe", s2 = "acd"
 * - 输出：false 
 * - 解释：s1="abe" 的所有排列包括："abe"，"aeb"，"bae"，"bea"，"eab" 和 "eba" ，s2="acd"
 * 的所有排列包括："acd"，"adc"，"cad"，"cda"，"dac" 和 "dca"。然而没有任何 s1 的排列可以打破 s2 的排列。也没有
 * s2 的排列能打破 s1 的排列。
 * 
 * 
 * ## 示例 3：
 * 
 * - 输入：s1 = "leetcodee", s2 = "interview"
 * - 输出：true
 * 
 * 
 * ## 提示：
 * 
 * - s1.length == n
 * - s2.length == n
 * - 1 <= n <= 10^5
 * - 所有字符串都只包含小写英文字母。
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut sc1 = s1.chars()
            .into_iter().collect::<Vec<_>>();

        sc1.sort();

        let mut sc2 = s2.chars()
            .into_iter().collect::<Vec<_>>();

        sc2.sort();

        sc1.clone().into_iter()
            .zip(sc2.clone().into_iter())
            .all(|(c1, c2)| c1 <= c2)
            ||
        sc1.into_iter()
            .zip(sc2.into_iter())
            .all(|(c1, c2)| c1 >= c2)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::check_if_can_break(
                "abc".to_string(), 
                "xya".to_string()
            ), 
            true
        );
        assert_eq!(
            Solution::check_if_can_break(
                "abe".to_string(), 
                "acd".to_string()
            ), 
            false
        );
        assert_eq!(
            Solution::check_if_can_break(
                "leetcode".to_string(), 
                "interview".to_string()
            ), 
            true
        );
        assert_eq!(
            Solution::check_if_can_break(
                "szy".to_string(), 
                "cid".to_string()
            ), 
            true
        );
    }
}
