/*!
 * # [96.不同的二叉搜索树](https://leetcode.cn/problems/unique-binary-search-trees/description/)
 *
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * ## 难度
 *
 * - Medium (70.87%)
 * - Likes:    2320
 * - Dislikes: 0
 * - Total Accepted:    382.6K
 * - Total Submissions: 539.8K
 * - Testcase Example:  '3'
 *
 * ## 问题描述
 *
 * 给你一个整数 n ，求恰由 n 个节点组成且节点值从 1 到 n 互不相同的 二叉搜索树 有多少种？返回满足题意的二叉搜索树的种数。
 *
 *
 * ## 示例 1：
 *
 * - 输入：n = 3
 * - 输出：5
 *
 *
 * ## 示例 2：
 *
 * - 输入：n = 1
 * - 输出：1
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 不同的二叉搜索树
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i]: 整数[1..=i]组成的不同二叉搜索树数;
    ///       f[j]: 以j为根节点的以[1..=n]组成的不同二叉搜索树数;
    /// 2. 则 dp[i] = f[1] + f[2] + .. + f[i];
    ///      f[i] = dp[i-1] * dp[n-i]
    ///    故递推关系为:
    ///      dp[n] = dp[0]*dp[n-1] + dp[1]*dp[n-2] + .. + dp[n-1]*dp[0]
    /// 3. 初始条件:
    ///      dp[0] = 1
    ///      dp[1] = 1
    /// 4. 目标: dp[n]
    pub fn num_trees(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let n = n as usize;
        let mut dp = vec![0_i32; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}
// @lc code=end
