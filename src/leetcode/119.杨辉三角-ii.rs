/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 *
 * https://leetcode.cn/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (68.92%)
 * Likes:    502
 * Dislikes: 0
 * Total Accepted:    277.7K
 * Total Submissions: 402.9K
 * Testcase Example:  '3'
 *
 * 给定一个非负索引 rowIndex，返回「杨辉三角」的第 rowIndex 行。
 *
 * 在「杨辉三角」中，每个数是它左上方和右上方的数的和。
 *
 *
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: rowIndex = 3
 * 输出: [1,3,3,1]
 *
 *
 * 示例 2:
 *
 *
 * 输入: rowIndex = 0
 * 输出: [1]
 *
 *
 * 示例 3:
 *
 *
 * 输入: rowIndex = 1
 * 输出: [1,1]
 *
 *
 *
 *
 * 提示:
 *
 *
 * 0
 *
 *
 *
 *
 * 进阶：
 *
 * 你可以优化你的算法到 O(rowIndex) 空间复杂度吗？
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 递归
    pub fn get_row(row_index: i32) -> Vec<i32> {
        use std::iter;

        match row_index {
            0 => vec![1],
            i => iter::once(1)
                .chain(Solution::get_row(i - 1).windows(2).map(|w| w.iter().sum()))
                .chain(iter::once(1))
                .collect::<Vec<_>>(),
        }
    }
}
// @lc code=end
