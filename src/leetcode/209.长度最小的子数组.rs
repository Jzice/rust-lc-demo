/*!
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * # [209] 长度最小的子数组
 *
 * https://leetcode.cn/problems/minimum-size-subarray-sum/description/
 *
 * algorithms
 * Medium (46.42%)
 * Likes:    2019
 * Dislikes: 0
 * Total Accepted:    671.9K
 * Total Submissions: 1.4M
 * Testcase Example:  '7\n[2,3,1,2,4,3]'
 *
 * 给定一个含有 n 个正整数的数组和一个正整数 target 。
 *
 * 找出该数组中满足其总和大于等于 target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr]
 * ，并返回其长度。如果不存在符合条件的子数组，返回 0 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：target = 7, nums = [2,3,1,2,4,3]
 * 输出：2
 * 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
 *
 *
 * 示例 2：
 *
 *
 * 输入：target = 4, nums = [1,4,4]
 * 输出：1
 *
 *
 * 示例 3：
 *
 *
 * 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= target <= 10^9
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^5
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # [209] 长度最小的子数组
    /// ## 解题思路
    /// 1. 双指针
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = nums.len() + 1;
        let (mut i, mut j, mut s) = (0, 0, 0);
        while j < nums.len() {
            while j < nums.len() && s < target {
                s += nums[j];
                j += 1;
            }
            while s >= target {
                res = res.min(j - i);
                s -= nums[i];
                i += 1;
            }
        }
        if res == nums.len() + 1 {
            res = 0;
        }

        res as i32
    }
}
// @lc code=end
