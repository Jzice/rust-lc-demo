/*!
 * [264.丑数II](https://leetcode.cn/problems/ugly-number-ii/description/)
 *
 * @lc app=leetcode.cn id=264 lang=rust
 *
 *
 * ## 状态
 * algorithms
 * Medium (58.57%)
 * Likes:    1097
 * Dislikes: 0
 * Total Accepted:    161.5K
 * Total Submissions: 275.8K
 * Testcase Example:  '10'
 *
 * ## 问题描述
 *
 * 给你一个整数 n ，请你找出并返回第 n 个 丑数 。
 *
 * 丑数 就是只包含质因数 2、3 和/或 5 的正整数。
 *
 *
 *
 * ## 示例 1：
 *
 * 输入：n = 10
 * 输出：12
 * 解释：[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] 是由前 10 个丑数组成的序列。
 *
 *
 * ## 示例 2：
 *
 * 输入：n = 1
 * 输出：1
 * 解释：1 通常被视为丑数。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 数学法
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let (mut c2, mut c3, mut c5) = (0, 0, 0);
        let mut dp = vec![1; n];
        for i in 1..n {
            dp[i] = vec![dp[c2] * 2, dp[c3] * 3, dp[c5] * 5]
                .into_iter()
                .min()
                .unwrap();
            if dp[i] == dp[c2] * 2 {
                c2 += 1;
            }
            if dp[i] == dp[c3] * 3 {
                c3 += 1;
            }
            if dp[i] == dp[c5] * 5 {
                c5 += 1;
            }
        }
        dp[n - 1]
    }
}
// @lc code=end
