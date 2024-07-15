/*!
 * # [136.只出现一次的数字](https://leetcode.cn/problems/single-number/description/)
 *
 * @lc app=leetcode.cn id=136 lang=rust
 *
 * ## 难度
 *
 * - Easy (72.34%)
 * - Likes:    2926
 * - Dislikes: 0
 * - Total Accepted:    932.7K
 * - Total Submissions: 1.3M
 * - Testcase Example:  '[2,2,1]'
 *
 *
 * ## 问题描述
 *
 * 给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
 *
 * 你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。
 *
 *
 * ## 示例 1 ：
 *
 * - 输入：nums = [2,2,1]
 * - 输出：1
 *
 *
 * ## 示例 2 ：
 *
 * - 输入：nums = [4,1,2,1,2]
 * - 输出：4
 *
 *
 * ## 示例 3 ：
 *
 * - 输入：nums = [1]
 * - 输出：1
 *
 *
 * ## 提示：
 *
 * - 1 <= nums.length <= 3 * 10^4
 * - -3 * 10^4 <= nums[i] <= 3 * 10^4
 * - 除了某个元素只出现一次以外，其余每个元素均出现两次。
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 只出现一次的数字
    /// ## 解题思路
    /// - 异或
    /// 1. a ^ a = 0
    /// 2. a ^ 0 = a
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |mut acc, &i| {
            acc ^= i;
            acc
        })
    }
}
// @lc code=end
