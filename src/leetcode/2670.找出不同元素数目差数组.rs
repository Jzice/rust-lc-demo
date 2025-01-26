/*!
 * # [2670.找出不同元素数目差数组](https://leetcode.cn/problems/find-the-distinct-difference-array/description/)
 *
 * @lc app=leetcode.cn id=2670 lang=rust slug=find-the-distinct-difference-array
 *
 * ## 难度
 *
 * Easy (82.83%)
 *
 * ## 问题描述 
 *
 * 给你一个下标从 0 开始的数组 nums ，数组长度为 n 。
 * nums 的 不同元素数目差 数组可以用一个长度为 n 的数组 diff 表示，其中 diff[i] 等于前缀 nums[0, ..., i] 中不同元素的数目 减去 后缀 nums[i + 1, ..., n - 1] 中不同元素的数目。
 * 返回 nums 的 不同元素数目差 数组。
 * 注意 nums[i, ..., j] 表示 nums 的一个从下标 i 开始到下标 j 结束的子数组（包含下标 i 和 j 对应元素）。特别需要说明的是，如果 i > j ，则 nums[i, ..., j] 表示一个空子数组。
 *  
 * ## 示例 1：
 *
 * - 输入：nums = [1,2,3,4,5]
 * - 输出：[-3,-1,1,3,5]
 * - 解释：
 *   - 对于 i = 0，前缀中有 1 个不同的元素，而在后缀中有 4 个不同的元素。因此，diff[0] = 1 - 4 = -3 。
 *   - 对于 i = 1，前缀中有 2 个不同的元素，而在后缀中有 3 个不同的元素。因此，diff[1] = 2 - 3 = -1 。
 *   - 对于 i = 2，前缀中有 3 个不同的元素，而在后缀中有 2 个不同的元素。因此，diff[2] = 3 - 2 = 1 。
 *   - 对于 i = 3，前缀中有 4 个不同的元素，而在后缀中有 1 个不同的元素。因此，diff[3] = 4 - 1 = 3 。
 *   - 对于 i = 4，前缀中有 5 个不同的元素，而在后缀中有 0 个不同的元素。因此，diff[4] = 5 - 0 = 5 。
 *
 * ## 示例 2：
 *
 * - 输入：nums = [3,2,3,4,2]
 * - 输出：[-2,-1,0,2,3]
 * - 解释：
 *   - 对于 i = 0，前缀中有 1 个不同的元素，而在后缀中有 3 个不同的元素。因此，diff[0] = 1 - 3 = -2 。
 *   - 对于 i = 1，前缀中有 2 个不同的元素，而在后缀中有 3 个不同的元素。因此，diff[1] = 2 - 3 = -1 。
 *   - 对于 i = 2，前缀中有 2 个不同的元素，而在后缀中有 2 个不同的元素。因此，diff[2] = 2 - 2 = 0 。
 *   - 对于 i = 3，前缀中有 3 个不同的元素，而在后缀中有 1 个不同的元素。因此，diff[3] = 3 - 1 = 2 。
 *   - 对于 i = 4，前缀中有 3 个不同的元素，而在后缀中有 0 个不同的元素。因此，diff[4] = 3 - 0 = 3 。 
 *  
 * ## 提示：
 *
 * - 1 <= n == nums.length <= 50
 * - 1 <= nums[i] <= 50
 *
 * ## 测试用例
 *
 * ```text
 *  [1,2,3,4,5]
 * ```
 */
use super::*;
// @lc code=start
impl Solution {
    /// # 2670.找出不同元素数目差数组 
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut prefix_set = std::collections::HashSet::new();
        let mut suffix_set = std::collections::HashSet::new();
        let mut prefix_count = vec![0; n];
        let mut suffix_count = vec![0; n];

        // 计算前缀不同元素数目
        for i in 0..n {
            prefix_set.insert(nums[i]);
            prefix_count[i] = prefix_set.len();
        }

        // 计算后缀不同元素数目
        for i in (0..n).rev() {
            suffix_set.insert(nums[i]);
            suffix_count[i] = suffix_set.len();
        }

        // 计算不同元素数目差数组
        for i in 0..n {
            let suffix_diff = if i + 1 < n { suffix_count[i + 1] } else { 0 };
            ans[i] = prefix_count[i] as i32 - suffix_diff as i32;
        }

        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_difference_array() {
        assert_eq!(Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]), vec![-3, -1, 1, 3, 5]);
        assert_eq!(Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]), vec![-2, -1, 0, 2, 3]);
    }
}

