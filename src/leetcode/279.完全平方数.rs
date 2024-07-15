/*!
 * # [279.完全平方数](https://leetcode.cn/problems/perfect-squares/description/)
 *
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * ## 难度
 *
 * - Medium (66.22%)
 * - Likes:    1773
 * - Dislikes: 0
 * - Total Accepted:    423.8K
 * - Total Submissions: 639.7K
 * - Testcase Example:  '12'
 *
 * ## 问题描述
 *
 * 给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。
 *
 * 完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11
 * 不是。
 *
 *
 *
 * ## 示例 1：
 *
 * - 输入：n = 12
 * - 输出：3
 * - 解释：12 = 4 + 4 + 4
 *
 *
 * ## 示例 2：
 *
 * - 输入：n = 13
 * - 输出：2
 * - 解释：13 = 4 + 9
 *
 *
 * 提示：
 *
 * - 1 <= n <= 10^4
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 完全平方数
    /// ## 解题思路
    /// - bfs
    pub fn num_squares1(n: i32) -> i32 {
        use std::collections::VecDeque;

        let mut res = 0;
        let mut q = VecDeque::new();
        q.push_back(n);
        let found = false;
        while !found && !q.is_empty() {
            res += 1;
            for _ in 0..q.len() {
                let n = q.pop_front().unwrap();
                for i in (1..=((n as f32).sqrt().floor() as i32))
                    .map(|i| i * i)
                    .filter(|&i| i <= n)
                {
                    if i == n {
                        return res;
                    }
                    q.push_back(n - i);
                }
            }
        }

        res
    }

    /// # 完全平方数
    /// - 动态规划
    /// 1. 设: dp[i]: i的完全平方数个数
    /// 2. 目标: dp[n]
    /// 3. dp[i] = min(dp[j]) + 1 ( 0<j<i, 且(i-j)为完全平方数)
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = (0..n + 1).collect::<Vec<_>>();
        for i in 2..=(n as usize) {
            dp[i] = (1..=(i as f32).sqrt().floor() as usize)
                .map(|j| dp[i - j * j] + 1)
                .min()
                .unwrap();
        }

        dp[n as usize]
    }
}
// @lc code=end
