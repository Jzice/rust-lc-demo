/*!
 * # [491.递增子序列](https://leetcode.cn/problems/non-decreasing-subsequences/description/)
 *
 * @lc app=leetcode.cn id=491 lang=rust
 *
 * ## 难度
 * - Medium (52.04%)
 * - Likes:    681
 * - Dislikes: 0
 * - Total Accepted:    141.5K
 * - Total Submissions: 272.1K
 * - Testcase Example:  '[4,6,7,7]'
 *
 * ## 问题描述
 *
 * 给你一个整数数组 nums ，找出并返回所有该数组中不同的递增子序列，递增子序列中 至少有两个元素 。你可以按 任意顺序 返回答案。
 *
 * 数组中可能含有重复元素，如出现两个整数相等，也可以视作递增序列的一种特殊情况。
 *
 *
 * ## 示例 1：
 * - 输入：nums = [4,6,7,7]
 * - 输出：[[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
 *
 * ## 示例 2：
 * - 输入：nums = [4,4,3,2,1]
 * - 输出：[[4,4]]
 *
 * ## 提示：
 * - 1 <= nums.length <= 15
 * - -100 <= nums[i] <= 100
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - dfs
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        fn dfs(nums: &[i32], start: usize, inc_nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                return;
            }
            let mut level_used = HashSet::new();
            for j in start..nums.len() {
                // 排除重复的数字
                if j > start && nums[j] == nums[j - 1] || level_used.contains(&nums[j]) {
                    continue;
                }
                if inc_nums.len() == 0 || nums[j] >= *inc_nums.last().unwrap() {
                    level_used.insert(nums[j]);
                    inc_nums.push(nums[j]);
                    if inc_nums.len() > 1 {
                        res.push(inc_nums.clone());
                    }
                    dfs(nums, j + 1, inc_nums, res);
                    inc_nums.pop();
                }
            }
        }

        let mut res = vec![];
        let mut inc_nums = vec![];
        dfs(&nums, 0, &mut inc_nums, &mut res);

        res
    }
}
// @lc code=end
struct Solution;
