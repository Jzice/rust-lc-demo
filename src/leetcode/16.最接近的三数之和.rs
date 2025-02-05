/*!
 * # [16.最接近的三数之和](https://leetcode.cn/problems/3sum-closest/description/)
 *
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * ## 难度
 * - Medium (43.03%)
 * - Likes:    367
 * - Dislikes: 0
 * - Total Accepted:    76.1K
 * - Total Submissions: 176.7K
 * - Testcase Example:  '[-1,2,1,-4]\n1'
 *
 * ## 问题描述
 *
 * 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target
 * 最接近。返回这三个数的和。假定每组输入只存在唯一答案。
 *
 * 例如，给定数组 nums = [-1，2，1，-4], 和 target = 1.
 *
 * 与 target 最接近的三个数的和为 2. (-1 + 2 + 1 = 2).
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 最接近的三数之和
    /// - 双指针
    /// 1. 类似15.三数和的思路
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        //
        let len = nums.len();
        if len < 3 {
            return 0;
        }

        let mut nums = nums;
        nums.sort();
        let mut res = None;
        let mut diff = i32::MAX;

        let c_diff = |sum, target| -> i32 { (sum as i32 - target as i32).abs() };
    'outer: 
        for i in 0..len - 2 {
            let (mut l, mut r) = (i + 1, len - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if c_diff(sum, target) < diff {
                    diff = c_diff(sum, target);
                    res = Some(sum);
                    if diff == 0 {
                        break 'outer;
                    }
                }
                if sum > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        res.expect("no")
    }
}
// @lc code=end


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}
