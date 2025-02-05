/*!
 * # [523.连续的子数组和](https://leetcode.cn/problems/continuous-subarray-sum/description/)
 *
 * @lc app=leetcode.cn id=523 lang=rust
 *
 * ## 难度
 * - Medium (28.56%)
 * - Likes:    526
 * - Dislikes: 0
 * - Total Accepted:    98.6K
 * - Total Submissions: 345K
 * - Testcase Example:  '[23,2,4,6,7]\n6'
 *
 * ## 问题描述
 *
 * 给你一个整数数组 nums 和一个整数 k ，编写一个函数来判断该数组是否含有同时满足下述条件的连续子数组：
 * - 子数组大小 至少为 2 ，且
 * - 子数组元素总和为 k 的倍数。
 *
 * 如果存在，返回 true ；否则，返回 false 。
 *
 * 如果存在一个整数 n ，令整数 x 符合 x = n * k ，则称 x 是 k 的一个倍数。0 始终视为 k 的一个倍数。
 *
 *
 * ## 示例 1：
 * - 输入：nums = [23,2,4,6,7], k = 6
 * - 输出：true
 * - 解释：[2,4] 是一个大小为 2 的子数组，并且和为 6 。
 *
 * ## 示例 2：
 * - 输入：nums = [23,2,6,4,7], k = 6
 * - 输出：true
 * - 解释：[23, 2, 6, 4, 7] 是大小为 5 的子数组，并且和为 42 。
 * 42 是 6 的倍数，因为 42 = 7 * 6 且 7 是一个整数。
 *
 *
 * ## 示例 3：
 * - 输入：nums = [23,2,6,4,7], k = 13
 * - 输出：false
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 连续的子数组和(dp)
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: nums[i..=j]的连续子数组和; (j>i+1)
    /// 2. 递推关系: dp[i][j+1] = dp[i][j] + nums[j]
    /// 3. 初始条件:
    ///          dp[i][i] = nums[i];
    pub fn check_subarray_sum1(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 0..(n - 1) {
            dp[i][i] = nums[i];
            for j in i..(n - 1) {
                dp[i][j + 1] = dp[i][j] + nums[j + 1];
                if dp[i][j + 1] % k == 0 {
                    return true;
                }
            }
        }

        false
    }

    /// # 连续的子数组和(前缀和)
    /// 1. 前缀和 prefix_sum[i]: sum(nums[..i]);
    /// 2. 连续子数组和: sub_sum[i][j] = sum(nums[i..j]) = prefix_sum[j] - prefix[i]
    /// 3. 如果 sub_sum 为 k 的倍数, 则 prefix_sum[j] % k == prefix_sum[i] % k;
    /// 4. 使用hashmap保存 (prefix_sum %k, i), 当存在同余的前缀和, 且下标差>1时, 存在k倍的子数组和;
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut prefix_sum = 0;
        let mut map = HashMap::new();
        map.insert(0, -1);
        for (i, &n) in nums.iter().enumerate() {
            prefix_sum += n;
            let pre_i = *map.entry(prefix_sum % k).or_insert(i as i32);
            if (i as i32) - pre_i > 1 {
                return true;
            }
        }

        false
    }
}
// @lc code=end
