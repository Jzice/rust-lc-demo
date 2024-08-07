/*!
 * # [461.汉明距离](https://leetcode.cn/problems/hamming-distance/description/)
 *
 * @lc app=leetcode.cn id=461 lang=rust
 *
 * ## 难度
 * Easy (81.29%)
 * Likes:    545
 * Dislikes: 0
 * Total Accepted:    179.6K
 * Total Submissions: 221K
 * Testcase Example:  '1\n4'
 *
 * ## 问题描述
 *
 * 两个整数之间的**汉明距离**指的是这两个数字对应二进制位不同的位置的数目。
 * 
 * 给你两个整数 x 和 y，计算并返回它们之间的汉明距离。
 * 
 * ## 示例 1：
 * - 输入：x = 1, y = 4
 * - 输出：2
 * - 解释：
 * ```text
 * 1   (0 0 0 1)
 * 4   (0 1 0 0)
 *       ↑   ↑
 * ```
 * 上面的箭头指出了对应二进制位不同的位置。
 * 
 * 
 * ## 示例 2：
 * - 输入：x = 3, y = 1
 * - 输出：1
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 汉明距离
    /// - 异或后检查是否包含1
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}
// @lc code=end

