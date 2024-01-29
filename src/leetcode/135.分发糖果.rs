/*
 * @lc app=leetcode.cn id=135 lang=rust
 *
 * [135] 分发糖果
 *
 * https://leetcode.cn/problems/candy/description/
 *
 * algorithms
 * Hard (50.42%)
 * Likes:    1265
 * Dislikes: 0
 * Total Accepted:    236.2K
 * Total Submissions: 468.5K
 * Testcase Example:  '[1,0,2]'
 *
 * n 个孩子站成一排。给你一个整数数组 ratings 表示每个孩子的评分。
 *
 * 你需要按照以下要求，给这些孩子分发糖果：
 *
 *
 * 每个孩子至少分配到 1 个糖果。
 * 相邻两个孩子评分更高的孩子会获得更多的糖果。
 *
 *
 * 请你给每个孩子分发糖果，计算并返回需要准备的 最少糖果数目 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：ratings = [1,0,2]
 * 输出：5
 * 解释：你可以分别给第一个、第二个、第三个孩子分发 2、1、2 颗糖果。
 *
 *
 * 示例 2：
 *
 *
 * 输入：ratings = [1,2,2]
 * 输出：4
 * 解释：你可以分别给第一个、第二个、第三个孩子分发 1、2、1 颗糖果。
 * ⁠    第三个孩子只得到 1 颗糖果，这满足题面中的两个条件。
 *
 *
 *
 * 提示：
 *
 *
 * n == ratings.length
 * 1 <= n <= 2 * 10^4
 * 0 <= ratings[i] <= 2 * 10^4
 *
 *
 */

use super::*;
struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 两次遍历
    /// 1.规则2等价于:
    ///    a. 自左至右遍历, 如果ratings[i] > ratings[i-1], 则当前糖果在前一人基础上+1;
    ///    b. 自右至左遍历, 如果ratings[i] > ratings[i+1], 则当前糖果在后一人基础上+1;
    ///   遍历玩后,选择每个人中糖果多的那个作为最后的分发的糖果
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy = 1;
        let mut left = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy += 1;
            } else {
                candy = 1;
            }
            left[i] = candy;
        }
        let mut total = 0;
        for i in (0..ratings.len()).rev() {
            if i < ratings.len() - 1 && ratings[i] > ratings[i + 1] {
                candy += 1;
            } else {
                candy = 1;
            }

            total += candy.max(left[i]);
        }

        total
    }
}
// @lc code=end
