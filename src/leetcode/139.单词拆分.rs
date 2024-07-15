/*!
 * # [139.单词拆分]( https://leetcode.cn/problems/word-break/description/)
 *
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * ## 难度
 *
 * - Medium (54.22%)
 * - Likes:    2231
 * - Dislikes: 0
 * - Total Accepted:    465.8K
 * - Total Submissions: 858.4K
 * - Testcase Example:  '"leetcode"\n["leet","code"]'
 *
 *
 * ## 问题描述
 *
 * 给你一个字符串 s 和一个字符串列表 wordDict 作为字典。请你判断是否可以利用字典中出现的单词拼接出 s 。
 *
 * 注意：不要求字典中出现的单词全部都使用，并且字典中的单词可以重复使用。
 *
 *
 * ## 示例 1：
 *
 * - 输入: s = "leetcode", wordDict = ["leet", "code"]
 * - 输出: true
 * - 解释: 返回 true 因为 "leetcode" 可以由 "leet" 和 "code" 拼接成。
 *
 *
 * ## 示例 2：
 *
 * - 输入: s = "applepenapple", wordDict = ["apple", "pen"]
 * - 输出: true
 * - 解释: 返回 true 因为 "applepenapple" 可以由 "apple" "pen" "apple" 拼接成。
 * 注意，你可以重复使用字典中的单词。
 *
 *
 * ## 示例 3：
 *
 * - 输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
 * - 输出: false
 *
 *
 * ## 提示：
 *
 * - 1 <= s.length <= 300
 * - 1 <= wordDict.length <= 1000
 * - 1 <= wordDict[i].length <= 20
 * - s 和 wordDict[i] 仅有小写英文字母组成
 * - wordDict 中的所有字符串 互不相同
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 单词拆分
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i]: 表示s[..i]是否能成功进行单词拆分;
    /// 2. 目标: dp[n] (n=s.len())
    /// 3. 递推关系:
    ///     dp[i+l] = true  ( dp[i] && s[i..(i+l)] in word_dict)
    /// 4. 初始条件:
    ///     dp[0] = true
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;
        let word_dict = word_dict.into_iter().collect::<HashSet<String>>();

        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 0..n {
            for l in word_dict.iter().map(|w| w.len()) {
                if dp[i] && i + l <= n && word_dict.contains(&s[i..(i + l)].to_string()) {
                    dp[i + l] = true;
                }
            }
        }

        dp[n]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::word_break("leetcode".into(), vec!["leet".into(), "code".into()]), true
        );

    }
}
