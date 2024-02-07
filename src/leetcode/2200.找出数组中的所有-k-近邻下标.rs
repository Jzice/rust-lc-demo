/*!
 * # [2200.找出数组中的所有K近邻下标](https://leetcode.cn/problems/find-all-k-distant-indices-in-an-array/description/)
 *
 * @lc app=leetcode.cn id=2200 lang=rust
 *
 * ## 难度
 * - Easy (53.79%)
 * - Likes:    15
 * - Dislikes: 0
 * - Total Accepted:    12.3K
 * - Total Submissions: 22.8K
 * - Testcase Example:  '[3,4,9,1,3,9,5]\n9\n1'
 *
 * ## 问题描述
 *
 * 给你一个下标从0开始的整数数组nums和两个整数key和k. K 近邻下标 是 nums 中的一个下标 i
 * ，并满足至少存在一个下标 j, 使得 |i - j| <= k 且 nums[j] == key 。
 *
 * 以列表形式返回按 递增顺序 排序的所有 K 近邻下标。
 *
 *
 * ## 示例 1：
 * - 输入：nums = [3,4,9,1,3,9,5], key = 9, k = 1
 * - 输出：[1,2,3,4,5,6]
 * - 解释：因此，nums[2] == key 且 nums[5] == key 。
 *   - 对下标 0 ，|0 - 2| > k 且 |0 - 5| > k ，所以不存在 j 使得 |0 - j| <= k 且 nums[j] == key. 所以 0 不是一个 K 近邻下标。
 *   - 对下标 1 ，|1 - 2| <= k 且 nums[2] == key ，所以 1 是一个 K 近邻下标。
 *   - 对下标 2 ，|2 - 2| <= k 且 nums[2] == key ，所以 2 是一个 K 近邻下标。
 *   - 对下标 3 ，|3 - 2| <= k 且 nums[2] == key ，所以 3 是一个 K 近邻下标。
 *   - 对下标 4 ，|4 - 5| <= k 且 nums[5] == key ，所以 4 是一个 K 近邻下标。
 *   - 对下标 5 ，|5 - 5| <= k 且 nums[5] == key ，所以 5 是一个 K 近邻下标。
 *   - 对下标 6 ，|6 - 5| <= k 且 nums[5] == key ，所以 6 是一个 K 近邻下标。
 * 因此，按递增顺序返回 [1,2,3,4,5,6] 。
 *
 *
 * ## 示例 2：
 * - 输入：nums = [2,2,2,2,2], key = 2, k = 2
 * - 输出：[0,1,2,3,4]
 * - 解释：对 nums 的所有下标 i ，总存在某个下标 j 使得 |i - j| <= k 且 nums[j] == key ，所以每个下标都是一个 K 近邻下标。因此，返回 [0,1,2,3,4] 。
 *
 *
 * ## 提示：
 * - 1 <= nums.length <= 1000
 * - 1 <= nums[i] <= 1000
 * - key 是数组 nums 中的一个整数
 * - 1 <= k <= nums.length
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 找出数组中的所有K近邻下标
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mut i = 0_i32;
        for (j, &c) in nums.iter().enumerate() {
            let j = j as i32;
            if c == key {
                while i < j - k {
                    i += 1;
                }
                while i <= j + k && i < (nums.len() as i32) {
                    res.push(i);
                    i += 1;
                }
            }
        }

        res
    }
}
// @lc code=end

