/*!
 * # [167.两数之和II-输入有序数组](https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/description/)
 *
 * @lc app=leetcode.cn id=167 lang=rust
 *
 * ## 难度
 * - Medium (59.96%)
 * - Likes:    1167
 * - Dislikes: 0
 * - Total Accepted:    640.8K
 * - Total Submissions: 1.1M
 * - Testcase Example:  '[2,7,11,15]\n9'
 *
 * ## 问题描述
 *
 * 给你一个下标从 1 开始的整数数组 numbers ，该数组已按 非递减顺序排列  ，请你从数组中找出满足相加之和等于目标数 target
 * 的两个数。如果设这两个数分别是 numbers[index1] 和 numbers[index2] ，则 1 <= index1 < index2 <=
 * numbers.length 。
 *
 * 以长度为 2 的整数数组 [index1, index2] 的形式返回这两个整数的下标 index1 和 index2。
 *
 * 你可以假设每个输入 只对应唯一的答案 ，而且你 不可以 重复使用相同的元素。
 *
 * 你所设计的解决方案必须只使用常量级的额外空间。
 *
 *
 * ## 示例 1：
 * - 输入：numbers = [2,7,11,15], target = 9
 * - 输出：[1,2]
 * - 解释：2 与 7 之和等于目标数 9 。因此 index1 = 1, index2 = 2 。返回 [1, 2] 。
 *
 * ## 示例 2：
 * - 输入：numbers = [2,3,4], target = 6
 * - 输出：[1,3]
 * - 解释：2 与 4 之和等于目标数 6 。因此 index1 = 1, index2 = 3 。返回 [1, 3] 。
 *
 * ## 示例 3：
 * - 输入：numbers = [-1,0], target = -1
 * - 输出：[1,2]
 * - 解释：-1 与 0 之和等于目标数 -1 。因此 index1 = 1, index2 = 2 。返回 [1, 2] 。
 *
 * ## 提示：
 * - 2 <= numbers.length <= 3 * 10^4
 * - -1000 <= numbers[i] <= 1000
 * - numbers 按 非递减顺序 排列
 * - -1000 <= target <= 1000
 * - 仅存在一个有效答案
 *
 */

use super::*;
struct Solution;

// @lc code=start
impl Solution {
    /// # 两数之和II-输入有序数组
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            if numbers[l] + numbers[r] == target {
                return vec![l as i32 + 1, r as i32 + 1];
            } else if numbers[l] + numbers[r] > target {
                let m = l + (r - l) / 2;
                if numbers[l] + numbers[m] == target {
                    return vec![l as i32 + 1, m as i32 + 1];
                } else if numbers[l] + numbers[m] > target {
                    r = m;
                } else {
                    r -= 1;
                }
            } else {
                let m = l + (r - l) / 2;
                if numbers[m] + numbers[r] == target {
                    return vec![m as i32 + 1, r as i32 + 1];
                } else if numbers[m] + numbers[r] < target {
                    l = m;
                } else {
                    l += 1;
                }
            }
        }
        return vec![];
    }
}
// @lc code=end
