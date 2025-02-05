/*!
 * # [441.排列硬币](https://leetcode.cn/problems/arranging-coins/description/)
 *
 * @lc app=leetcode.cn id=441 lang=rust
 *
 * ## 难度
 * - Easy (45.46%)
 * - Likes:    262
 * - Dislikes: 0
 * - Total Accepted:    114.7K
 * - Total Submissions: 252.3K
 * - Testcase Example:  '5'
 *
 * ## 问题描述
 *
 * 你总共有 n 枚硬币，并计划将它们按阶梯状排列。对于一个由 k 行组成的阶梯，其第 i 行必须正好有 i 枚硬币。阶梯的最后一行 可能 是不完整的。
 *
 * 给你一个数字 n ，计算并返回可形成 完整阶梯行 的总行数。
 *
 * ## 示例 1：
 * - 输入：n = 5
 * - 输出：2
 * - 解释：因为第三行不完整，所以返回 2 。
 *
 * ## 示例 2：
 * - 输入：n = 8
 * - 输出：3
 * - 解释：因为第四行不完整，所以返回 3 。
 *
 * ## 提示：
 * - 1 <= n <= 2^31 - 1
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// ### 二分查找
    /// - 设i为行数,则对于所有行为满的排列，
    ///     总数 `n = i * (i+1) / 2`
    /// - 对于不满最后一行的n，有：
    ///     (i-1) * i / 2 < n < i*(i+1)/2
    ///   => i*i - i  < 2n < i*i + i
    pub fn arrange_coins(n: i32) -> i32 {
        let (mut l, mut r) = (1, n);
        while l < r {
            let m = (l + r + 1) / 2;
            if m * m + m <= 2 * n {
                l = m;
            } else {
                r = m - 1;
            }
        }

        l
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::arrange_coins(8), 3);
    }
}
