/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 *
 * https://leetcode.cn/problems/majority-element/description/
 *
 * algorithms
 * Easy (66.87%)
 * Likes:    1720
 * Dislikes: 0
 * Total Accepted:    661.3K
 * Total Submissions: 989.6K
 * Testcase Example:  '[3,2,3]'
 *
 * 给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。
 *
 * 你可以假设数组是非空的，并且给定的数组总是存在多数元素。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [3,2,3]
 * 输出：3
 *
 * 示例 2：
 *
 *
 * 输入：nums = [2,2,1,1,1,2,2]
 * 输出：2
 *
 *
 *
 * 提示：
 *
 *
 * n == nums.length
 * 1 <= n <= 5 * 10^4
 * -10^9 <= nums[i] <= 10^9
 *
 *
 *
 *
 * 进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - Boyer-Moore 算法
    /// 1. 设置两个计数: candidate=0，count=0;
    /// 2. 依次遍历nums;
    /// 3. 若count为0，则 candidate = n;
    /// 4. 再在 `candidate == n`时, count += 1
    ///        `candidate != n`时，count -= 1
    /// 5. 遍历完成后， candidate即为多数元素。
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        nums.iter().for_each(|&n| {
            if count == 0 {
                candidate = n;
            }
            if candidate == n {
                count += 1;
            } else {
                count -= 1;
            }
        });

        candidate
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
