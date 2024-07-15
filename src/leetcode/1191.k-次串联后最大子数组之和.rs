/*
 * @lc app=leetcode.cn id=1191 lang=rust
 *
 * [1191] K 次串联后最大子数组之和
 *
 * https://leetcode.cn/problems/k-concatenation-maximum-sum/description/
 *
 * algorithms
 * Medium (26.97%)
 * Likes:    112
 * Dislikes: 0
 * Total Accepted:    8.6K
 * Total Submissions: 31.8K
 * Testcase Example:  '[1,2]\n3'
 *
 * 给定一个整数数组 arr 和一个整数 k ，通过重复 k 次来修改数组。
 *
 * 例如，如果 arr = [1, 2] ， k = 3 ，那么修改后的数组将是 [1, 2, 1, 2, 1, 2] 。
 *
 * 返回修改后的数组中的最大的子数组之和。注意，子数组长度可以是 0，在这种情况下它的总和也是 0。
 *
 * 由于 结果可能会很大，需要返回的 10^9 + 7 的 模 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：arr = [1,2], k = 3
 * 输出：9
 *
 *
 * 示例 2：
 *
 *
 * 输入：arr = [1,-2,1], k = 5
 * 输出：2
 *
 *
 * 示例 3：
 *
 *
 * 输入：arr = [-1,-2], k = 7
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 *
 * 1 <= arr.length <= 10^5
 * 1 <= k <= 10^5
 * -10^4 <= arr[i] <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 当k=1时, 问题退化为`53.最大连续子数组和`, 可通过动态规划解决;
    /// 2. 当k>1时, 先求出前面2个完整arr组合数组(重复2次)的最大子数组和max_sub_sum;
    /// 3. 再计算单个数组arr的总和sum, 如果sum > 0,
    ///    可在max_sub_sum后重复串接完整的arr循环得到更大的连续子数组和;
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut sub_sum = arr[0];  //当前以arr[i]为尾的最大子数组和
        let mut max_sub_sum = sub_sum as i64; //最大子数组和
        let min_k = k.min(2);

        // 前2个重复周期中的最大子数组和
        for i in 1..((min_k as usize) * n) {
            let i = i % n;
            sub_sum = arr[i].max(sub_sum + arr[i]);
            max_sub_sum = max_sub_sum.max(sub_sum as i64);
        }

        // 整个数组和
        let sum = arr.iter().clone().sum::<i32>(); 
        // 如果整个数组和>0
        if sum > 0 {
            // 则可在当个最大和子数组上重复串连完整的数组
            max_sub_sum += sum as i64 * ((k - min_k) as i64);
        }
        if max_sub_sum < 0 {
            max_sub_sum = 0;
        }

        (max_sub_sum % 1000000008_i64) as i32
    }
}
// @lc code=end
