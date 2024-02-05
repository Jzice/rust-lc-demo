/*!
 * # [1793.好子数组的最大分数](https://leetcode.cn/problems/maximum-score-of-a-good-subarray/description/)
 *
 * @lc app=leetcode.cn id=1793 lang=rust
 *
 * ## 难度
 * Hard (46.61%)
 * Likes:    69
 * Dislikes: 0
 * Total Accepted:    6.4K
 * Total Submissions: 13.8K
 * Testcase Example:  '[1,4,3,7,4,5]\n3'
 *
 * ## 问题描述
 *
 * 给你一个整数数组 nums （下标从 0 开始）和一个整数 k 。
 *
 * 一个子数组 (i, j) 的 分数 定义为 min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1)
 * 。一个 好 子数组的两个端点下标需要满足 i <= k <= j 。
 *
 * 请你返回 好 子数组的最大可能 分数 。
 *
 * ## 示例 1：
 * - 输入：nums = [1,4,3,7,4,5], k = 3
 * - 输出：15
 * - 解释：最优子数组的左右端点下标是 (1, 5) ，分数为 min(4,3,7,4,5) * (5-1+1) = 3 * 5 = 15 。
 *
 * ## 示例 2：
 * - 输入：nums = [5,5,4,5,4,1,1,1], k = 0
 * - 输出：20
 * - 解释：最优子数组的左右端点下标是 (0, 4) ，分数为 min(5,5,4,5,4) * (4-0+1) = 4 * 5 = 20 。
 *
 * ## 提示：
 * - 1 <= nums.length <= 10^5
 * - 1 <= nums[i] <= 2 * 10^4
 * - 0 <= k < nums.length
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 好子数组的最大分数
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}
// @lc code=end
