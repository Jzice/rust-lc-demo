/*!
 * # [11.盛最多水的容器](https://leetcode.cn/problems/container-with-most-water/description/)
 *
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * ## 难度
 * - Medium (61.40%)
 * - Likes:    1147
 * - Dislikes: 0
 * - Total Accepted:    143.9K
 * - Total Submissions: 234.1K
 * - Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * ## 问题描述
 *
 * 给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为
 * (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
 *
 * 说明：你不能倾斜容器，且 n 的值至少为 2。
 *
 * 图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
 *
 * ## 示例:
 *
 * - 输入: [1,8,6,2,5,4,8,3,7]
 * - 输出: 49
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 盛最多水的容器
    /// ## 解题思路
    /// - 双指针法
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let (mut l, mut r) = (0 as usize, height.len() - 1);
        while l < r {
            area = area.max(std::cmp::min(height[l], height[r]) * ((r - l) as i32));
            if height[l] < height[r] {
                l += 1
            } else {
                r -= 1
            }
        }

        area
    }
}
// @lc code=end
