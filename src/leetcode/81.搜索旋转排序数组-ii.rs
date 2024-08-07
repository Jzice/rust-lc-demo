/*!
 * # [81.搜索旋转排序数组II]( https://leetcode.cn/problems/search-in-rotated-sorted-array-ii/description/)
 *
 * @lc app=leetcode.cn id=81 lang=rust
 *
 * ## 难度
 * - Medium (40.98%)
 * - Likes:    717
 * - Dislikes: 0
 * - Total Accepted:    199K
 * - Total Submissions: 485.7K
 * - Testcase Example:  '[2,5,6,0,0,1,2]\n0'
 *
 * ## 问题描述
 *
 * 已知存在一个按非降序排列的整数数组 nums ，数组中的值不必互不相同。
 *
 * 在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转 ，使数组变为 [nums[k],
 * nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始
 * 计数）。例如， [0,1,2,4,4,4,5,6,6,7] 在下标 5 处经旋转后可能变为 [4,5,6,6,7,0,1,2,4,4] 。
 *
 * 给你 旋转后 的数组 nums 和一个整数 target ，请你编写一个函数来判断给定的目标值是否存在于数组中。如果 nums 中存在这个目标值
 * target ，则返回 true ，否则返回 false 。
 *
 * 你必须尽可能减少整个操作步骤。
 *
 * ## 示例 1：
 * - 输入：nums = [2,5,6,0,0,1,2], target = 0
 * - 输出：true
 *
 * ## 示例 2：
 * - 输入：nums = [2,5,6,0,0,1,2], target = 3
 * - 输出：false
 *
 * ## 提示：
 * - 1 <= nums.length <= 5000
 * - -10^4 <= nums[i] <= 10^4
 * - 题目数据保证 nums 在预先未知的某个下标上进行了旋转
 * - -10^4 <= target <= 10^4
 *
 * ## 进阶：
 * 这是 搜索旋转排序数组 的延伸题目，本题中的 nums  可能包含重复元素。
 * 这会影响到程序的时间复杂度吗？会有怎样的影响，为什么？
 *
 */

// @lc code=start
impl Solution {
    /// # 搜索旋转排序数组II
    /// ## 解题思路
    /// - 二分查找
    /// 1. 在<<33.搜索旋转排序数组>>题目基础上, 增加对重复数字的判断;
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = (l + r + 1) / 2;
            if target == nums[l] || target == nums[r] || target == nums[m]  {
                return true;
            }
            if nums[m] == nums[l] { 
                l += 1;
            } else if nums[m] == nums[r] {
                r -= 1;
            } else if nums[m] > nums[l] {
                if target > nums[l] && target < nums[m] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if target > nums[m] && target < nums[r] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }

        return false;
    }
}
// @lc code=end

struct Solution;
