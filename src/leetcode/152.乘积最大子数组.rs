/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子数组
 *
 * https://leetcode.cn/problems/maximum-product-subarray/description/
 *
 * algorithms
 * Medium (43.20%)
 * Likes:    2078
 * Dislikes: 0
 * Total Accepted:    373K
 * Total Submissions: 863.4K
 * Testcase Example:  '[2,3,-2,4]'
 *
 * 给你一个整数数组 nums ，请你找出数组中乘积最大的非空连续子数组（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。
 *
 * 测试用例的答案是一个 32-位 整数。
 *
 * 子数组 是数组的连续子序列。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: nums = [2,3,-2,4]
 * 输出: 6
 * 解释: 子数组 [2,3] 有最大乘积 6。
 *
 *
 * 示例 2:
 *
 *
 * 输入: nums = [-2,0,-1]
 * 输出: 0
 * 解释: 结果不能为 2, 因为 [-2,-1] 不是子数组。
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= nums.length <= 2 * 10^4
 * -10 <= nums[i] <= 10
 * nums 的任何前缀或后缀的乘积都 保证 是一个 32-位 整数
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp_max[i]: 以nums[i]为尾的最大子数组积;
    ///       dp_min[i]: 以nums[i]为尾的最小子数组积;
    /// 2. 则 dp_max[i] = max(dp_max[i-1]*nums[i], dp_min[i-1]*nums[i], nums[i]);
    ///       dp_min[i] = min(dp_max[i-1]*nums[i], dp_min[i-1]*nums[i], nums[i]);
    /// 3. 初始条件:
    ///       dp_max[0] = nums[0];
    ///       dp_min[0] = nums[0];
    /// 4. 目标:
    ///       max(dp_max[])
    pub fn max_product0(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_max = vec![i32::MAX; n];
        let mut dp_min = vec![i32::MIN; n];
        dp_max[0] = nums[0];
        dp_min[0] = nums[0];
        for i in 1..n {
            dp_max[i] = nums[i]
                .max(dp_max[i - 1] * nums[i])
                .max(dp_min[i - 1] * nums[i]);
            dp_min[i] = nums[i]
                .min(dp_max[i - 1] * nums[i])
                .min(dp_min[i - 1] * nums[i]);
        }

        dp_max.into_iter().max().unwrap()
    }

    /// - 优化:
    ///   - dp_max[i]只和dp_max[i-1]有关系, 可用2个变量辗转代替;
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_max = nums[0];
        let mut dp_min = nums[0];
        let mut res = dp_max;
        for i in 1..n {
            let (last_max, last_min) = (dp_max, dp_min);
            dp_max = nums[i].max(last_max * nums[i]).max(last_min * nums[i]);
            dp_min = nums[i].min(last_max * nums[i]).min(last_min * nums[i]);
            res = res.max(dp_max);
        }

        res
    }
}
// @lc code=end
