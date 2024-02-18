/*!
 *
 * # [1186.删除一次得到子数组最大和](https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/description/)
 *
 * @lc app=leetcode.cn id=1186 lang=rust
 *
 * ## 难度
 * - Medium (48.11%)
 * - Likes:    287
 * - Dislikes: 0
 * - Total Accepted:    27.4K
 * - Total Submissions: 57K
 * - Testcase Example:  '[1,-2,0,3]'
 *
 * ## 描述
 *
 * 给你一个整数数组，返回它的某个 非空
 * 子数组（连续元素）在执行一次可选的删除操作后，所能得到的最大元素总和。换句话说，你可以从原数组中选出一个子数组，并可以决定要不要从中删除一个元素（只能删一次哦），（删除后）子数组中至少应当有一个元素，然后该子数组（剩下）的元素总和是所有子数组之中最大的。
 *
 * 注意，删除一个元素后，子数组 不能为空。
 *
 * ## 示例 1：
 * - 输入：arr = [1,-2,0,3]
 * - 输出：4
 * - 解释：我们可以选出 [1, -2, 0, 3]，然后删掉 -2，这样得到 [1, 0, 3]，和最大。
 *
 * ## 示例 2：
 * - 输入：arr = [1,-2,-2,3]
 * - 输出：3
 * - 解释：我们直接选出 [3]，这就是最大和。
 *
 *
 * ## 示例 3：
 * - 输入：arr = [-1,-1,-1,-1]
 * - 输出：-1
 * - 解释：最后得到的子数组不能为空，所以我们不能选择 [-1] 并从中删去 -1 来得到 0。
 * 我们应该直接选择 [-1]，或者选择 [-1, -1] 再从中删去一个 -1。
 *
 * ## 提示：
 * - 1 <= arr.length <= 10^5
 * - -10^4 <= arr[i] <= 10^4
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 sub_sum[i]: 以nums[i]为尾的连续子数组最大和
    ///       sub_sum_1[i]: 以nums[i]为尾的连续子数组删除一个元素得到的最大和
    /// 2. 则 sub_sum[i] = max(nums[i], sub_sum[i-1]+nums[i])
    ///       sub_sum_1[i] = max(nums[i], sub_sum_1[i-1]+nums[i], sub_sum[i-2]+nums[i])
    /// 3. 初始条件: sub_sum_1[0] = nums[0]
    ///             sub_sum_1[1] = max(nums[0], sub_sum_1[0] + nums[1])
    ///             sub_sum[0] = nums[0]
    ///             sub_sum[1] = max(sub_sum[0], sub_sum[0]+nums[1])
    /// 4. 目标: max(sub_sum[i])
    pub fn maximum_sum1(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 {
            return arr[0];
        }
        let mut sub_sum = vec![0; n];
        let mut sub_sum_1 = vec![0; n];
        sub_sum[0] = arr[0];
        sub_sum_1[0] = arr[0];
        sub_sum[1] = arr[1].max(arr[0] + arr[1]);
        sub_sum_1[1] = arr[1].max(arr[0] + arr[1]);

        let mut max_sub_sum_1 = sub_sum_1[0].max(sub_sum_1[1]);
        for i in 2..n {
            sub_sum_1[i] = arr[i]
                .max(sub_sum_1[i - 1] + arr[i])
                .max(sub_sum[i - 2] + arr[i]);
            sub_sum[i] = arr[i].max(sub_sum[i - 1] + arr[i]);
            max_sub_sum_1 = max_sub_sum_1.max(sub_sum_1[i]);
        }

        max_sub_sum_1
    }

    /// - 优化:
    ///   1. sub_sum[i] 只和sub_sum[i-1]相关, 可以用一个整形代替;
    ///   2. sub_sum_1[i] 只和sub_sum_1[i-1], sub_sum[i-2]相关, 也可以用一个整形代替;
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 {
            return arr[0];
        }
        let mut sub_sum = 0; //当前最大连续子数组和
        let mut sub_sum_1 = arr[1].max(arr[0] + arr[1]); //当前最大连续子数组删除1次和

        let mut max_sub_sum_1 = sub_sum_1;
        for i in 2..n {
            sub_sum = arr[i - 2].max(sub_sum + arr[i - 2]);
            sub_sum_1 = arr[i].max(sub_sum_1 + arr[i]).max(sub_sum + arr[i]);
            max_sub_sum_1 = max_sub_sum_1.max(sub_sum_1);
        }

        max_sub_sum_1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::maximum_sum(vec![1,-2,0,3]), 4);
        assert_eq!(Solution::maximum_sum(vec![1,-2,-2,3]), 3);
        assert_eq!(Solution::maximum_sum(vec![-1,-1,-1,-1]), -1);
    }
}
