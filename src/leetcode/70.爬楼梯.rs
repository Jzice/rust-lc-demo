/*!
 * # [70.爬楼梯](https://leetcode.cn/problems/climbing-stairs/description/)
 *
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * ## 难度
 *
 * - Easy (47.88%)
 * - Likes:    856
 * - Dislikes: 0
 * - Total Accepted:    139.1K
 * - Total Submissions: 290.2K
 * - Testcase Example:  '2'
 *
 *
 * ## 问题描述
 *
 * 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
 *
 * 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
 *
 * 注意：给定 n 是一个正整数。
 *
 * ## 示例 1：
 *
 * - 输入： 2
 * - 输出： 2
 * - 解释： 有两种方法可以爬到楼顶。
 *   1. 1 阶 + 1 阶
 *   2. 2 阶
 *
 * ## 示例 2：
 *
 * - 输入： 3
 * - 输出： 3
 * - 解释： 有三种方法可以爬到楼顶。
 *   1. 1 阶 + 1 阶 + 1 阶
 *   2. 1 阶 + 2 阶
 *   3. 2 阶 + 1 阶
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设f(n): 爬上第n级楼梯的总方法;
    /// 2. 则f(n-1), f(n-2)分别为爬上第n-1,n-2级台阶的方法
    /// 3. 爬上第n级台阶总有两种方式：
    ///     1. 从第n-1级台阶上1级；
    ///     2. 从第n-2机台阶上2级；
    /// 则：f(n) = f(n-1) + f(n-2)
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 | 2 => n,
            _ => {
                let (mut prev, mut current) = (1, 2);
                for _ in 3..n + 1 {
                    let old_curr = current;
                    current = prev + current;
                    prev = old_curr;
                }

                current
            }
        }
    }
}
// @lc code=end
