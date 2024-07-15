/*!
 * # [50.Pow(x, n)]( https://leetcode.cn/problems/powx-n/description/)
 *
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * ## 难度
 *
 * Medium (38.01%)
 * Likes:    1167
 * Dislikes: 0
 * Total Accepted:    370.3K
 * Total Submissions: 974.2K
 * Testcase Example:  '2.00000\n10'
 *
 *
 * ## 题目描述
 *
 * 实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，x^n^ ）。
 *
 *
 * ## 示例 1：
 *
 * - 输入：x = 2.00000, n = 10
 * - 输出：1024.00000
 *
 *
 * ## 示例 2：
 *
 * - 输入：x = 2.10000, n = 3
 * - 输出：9.26100
 *
 *
 * ## 示例 3：
 *
 * - 输入：x = 2.00000, n = -2
 * - 输出：0.25000
 * - 解释：2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *
 * ## 提示：
 *
 * - -100.0 < x < 100.0
 * - -2^31 <= n <= 2^31-1
 * - n 是一个整数
 * - -10^4 <= x^n <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 快速幂
    /// ## 解题思路
    /// - 二分
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match (x, n) {
            (_, 1) => x,
            (_, 0) => 1.0,
            (x, _) if x == 1.0 => 1.0,
            (x, _) if x == -1.0 => if n % 2 == 0 { 1.0 } else { -1.0 },
            _ => {
                let mut m = n.abs() as u32;
                let mut x = x;
                let mut res = 1.0;
                while m > 0 {
                    if m %2 != 0 {
                        res *= x;
                    } 
                    x *= x;
                    m /= 2;
                }

                if n < 0 {
                    1.0/res
                } else {
                    res
                }
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_pow(2.00000_f64, -2), 0.25000_f64);
    }
}
