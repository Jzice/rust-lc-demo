/*
 * @lc app=leetcode.cn id=162 lang=rust
 *
 * [162] 寻找峰值
 *
 * https://leetcode.cn/problems/find-peak-element/description/
 *
 * algorithms
 * Medium (49.31%)
 * Likes:    1113
 * Dislikes: 0
 * Total Accepted:    321K
 * Total Submissions: 651K
 * Testcase Example:  '[1,2,3,1]'
 *
 * 峰值元素是指其值严格大于左右相邻值的元素。
 *
 * 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
 *
 * 你可以假设 nums[-1] = nums[n] = -∞ 。
 *
 * 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,3,1]
 * 输出：2
 * 解释：3 是峰值元素，你的函数应该返回其索引 2。
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,1,3,5,6,4]
 * 输出：1 或 5
 * 解释：你的函数可以返回索引 1，其峰值元素为 2；
 * 或者返回索引 5， 其峰值元素为 6。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 1000
 * -2^31 <= nums[i] <= 2^31 - 1
 * 对于所有有效的 i 都有 nums[i] != nums[i + 1]
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分查找
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let r_max = nums.len() - 1;
        let (mut l, mut r) = (0, r_max);
        while l < r {
            let m = (l + r) >> 1;
            // 如果 nums[m]为左边界, 且>nums[m+1]
            //   或者 nums[m]为右边界, 且<nums[m-1]
            //   或者 不为边界元素, 且大于左右两边临近元素,
            // 则为峰值
            if (m == 0 && nums[m] > nums[m + 1])
                || (m == nums.len() - 1 && nums[m] > nums[m - 1])
                || ((1..r_max).contains(&m) && nums[m] > nums[m - 1] && nums[m] > nums[m + 1])
            {
                return m as i32;
            } else if (m == 0 && nums[m] < nums[m + 1])
                || ((1..r_max).contains(&m) && nums[m] > nums[m - 1] && nums[m] < nums[m + 1])
            {
                l = m + 1;
            } else if (m == nums.len() - 1 && nums[m] < nums[m - 1])
                || ((1..r_max).contains(&m) && nums[m] < nums[m - 1] && nums[m] > nums[m + 1])
            {
                r = m;
            } else {
                if nums[l] > nums[r] {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }

        l as i32
    }
}
// @lc code=end
