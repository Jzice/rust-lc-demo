/*!
 * # [115.不同的子序列](https://leetcode.cn/problems/distinct-subsequences/description/)
 * 
 * @lc app=leetcode.cn id=115 lang=rust
 *
 * algorithms
 * Hard (52.21%)
 * Likes:    1092
 * Dislikes: 0
 * Total Accepted:    145.8K
 * Total Submissions: 279.4K
 * Testcase Example:  '"rabbbit"\n"rabbit"'
 *
 * 给你两个字符串 s 和 t ，统计并返回在 s 的 子序列 中 t 出现的个数。
 *
 * 题目数据保证答案符合 32 位带符号整数范围。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "rabbbit", t = "rabbit"
 * 输出：3
 * 解释：
 * 如下所示, 有 3 种可以从 s 中得到 "rabbit" 的方案。
 * rabbbit
 * rabbbit
 * rabbbit
 *
 * 示例 2：
 *
 *
 * 输入：s = "babgbag", t = "bag"
 * 输出：5
 * 解释：
 * 如下所示, 有 5 种可以从 s 中得到 "bag" 的方案。
 * babgbag
 * babgbag
 * babgbag
 * babgbag
 * babgbag
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length, t.length <= 1000
 * s 和 t 由英文字母组成
 *
 *
 */

struct Solution;
 
// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: 表示s[..i], t[..j]的不同子序列个数;
    /// 2. 目标: dp[s.len()][t.len()];
    /// 3. 递推关系:
    ///       dp[i+1][j+1] = dp[i][j+1]             (s[i] != t[j], s[..i]和t[..j]不同子序列数等同于s[..(i-1)]和t[..j])
    ///                    = dp[i][j+1] + dp[i][j]  (s[i] == t[j], 此时将匹配情况分2类:
    ///                                                  a. 匹配子序列包含s[i]的, 个数有dp[i][j];
    ///                                                  b. 匹配子序列不包含s[i]的, 个数为dp[i][j+1];
    ///                                             )
    /// 4. 初始条件:
    ///       dp[i][0] = 1,  (t为空, s中和t匹配的子序列为1)
    pub fn num_distinct0(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (m, n) = (s.len(), t.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            dp[i][0] = 1;
            for j in 0..n {
                dp[i + 1][j + 1] = if s[i] == t[j] {
                    dp[i][j + 1] + dp[i][j]
                } else {
                    dp[i][j + 1]
                };
            }
        }

        dp[m][n]
    }

    /// - 优化:
    ///    dp[i+1][] 只和dp[i][]有关系, 用一维数组代替二维数组
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (m, n) = (s.len(), t.len());
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 0..m {
            for j in (0..n).rev() {
                if s[i] == t[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[n]
    }
}
// @lc code=end
