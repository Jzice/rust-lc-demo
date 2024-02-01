/*!
 * # [263].丑数](https://leetcode.cn/problems/ugly-number/description/)
 *
 * @lc app=leetcode.cn id=263 lang=rust
 *
 * ## 难度
 * - Easy (50.48%)
 * - Likes:    414
 * - Dislikes: 0
 * - Total Accepted:    161.2K
 * - Total Submissions: 319.4K
 * - Testcase Example:  '6'
 *
 * ## 问题描述
 *
 * 丑数 就是只包含质因数 2、3 和 5 的正整数。
 *
 * 给你一个整数 n ，请你判断 n 是否为 丑数 。如果是，返回 true ；否则，返回 false 。
 *
 *
 *
 * ## 示例 1：
 *
 * - 输入：n = 6
 * - 输出：true
 * - 解释：6 = 2 × 3
 *
 * ## 示例 2：
 *
 * - 输入：n = 1
 * - 输出：true
 * - 解释：1 没有质因数，因此它的全部质因数是 {2, 3, 5} 的空集。习惯上将其视作第一个丑数。
 *
 * ## 示例 3：
 *
 * - 输入：n = 14
 * - 输出：false
 * - 解释：14 不是丑数，因为它包含了另外一个质因数 7 。
 *
 * ## 提示：
 *
 * -2^31 <= n <= 2^31 - 1
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 丑数
    /// ## 解题思路
    /// - 递归
    pub fn is_ugly0(n: i32) -> bool {
        match n {
            0 => false,
            1 | 2 | 3 | 5 => true,
            _ => {
                (n % 2 == 0 && Solution::is_ugly(n / 2))
                    || (n % 3 == 0 && Solution::is_ugly(n / 3))
                    || (n % 5 == 0 && Solution::is_ugly(n / 5))
            }
        }
    }
    /// - 迭代
    pub fn is_ugly(n: i32) -> bool {
        match n {
            0 => false,
            1 | 2 | 3 | 5 => true,
            mut n => {
                for p in [2, 3, 5] {
                    while n % p == 0 {
                        n /= p;
                    }
                }
                n == 1
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
        assert_eq!(Solution::is_ugly0(6), true);
        assert_eq!(Solution::is_ugly0(1), true);
        assert_eq!(Solution::is_ugly0(14), false);

        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(14), false);

    }
}
