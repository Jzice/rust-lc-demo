/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 *
 * https://leetcode.cn/problems/sort-colors/description/
 *
 * algorithms
 * Medium (60.43%)
 * Likes:    1611
 * Dislikes: 0
 * Total Accepted:    543.5K
 * Total Submissions: 899.4K
 * Testcase Example:  '[2,0,2,1,1,0]'
 *
 * 给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
 *
 * 我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
 *
 *
 *
 *
 * 必须在不使用库内置的 sort 函数的情况下解决这个问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,0,2,1,1,0]
 * 输出：[0,0,1,1,2,2]
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [2,0,1]
 * 输出：[0,1,2]
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == nums.length
 * 1 <= n <= 300
 * nums[i] 为 0、1 或 2
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 你能想出一个仅使用常数空间的一趟扫描算法吗？
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 双指针
    /// [ 0, 0, 1, 0, 2, 0, 1, 0, 2, 1 ]
    /// [ 0, 0, 0, 0, 0, 1, 1, 1, 2, 2 ]
    ///                off_1    off_2
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut off_1, mut off_2) = (0, 0, nums.len());
        while i < off_2 {
            match nums[i] {
                0 => {
                    nums.swap(i, off_1); // 将当前的0放置到0区间后面
                    off_1 += 1; // 0区间元素增多, 1区间左边界右移
                    i += 1; //
                }
                2 => {
                    off_2 -= 1; // 移动2区间左边界
                    nums.swap(i, off_2); // 将当前的2移动到2区间左边
                                         // nums[off_2]交换前的数没有访问过, 所以i不递增
                }
                _ => {
                    i += 1; // 当前元素为1,
                }
            }
        }
    }
}
// @lc code=end

struct Solution;
