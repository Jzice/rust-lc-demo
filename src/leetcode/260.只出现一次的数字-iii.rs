/*
 * @lc app=leetcode.cn id=260 lang=rust
 *
 * [260] 只出现一次的数字 III
 *
 * https://leetcode.cn/problems/single-number-iii/description/
 *
 * algorithms
 * Medium (71.89%)
 * Likes:    731
 * Dislikes: 0
 * Total Accepted:    115.1K
 * Total Submissions: 160.4K
 * Testcase Example:  '[1,2,1,3,2,5]'
 *
 * 给你一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。
 *
 * 你必须设计并实现线性时间复杂度的算法且仅使用常量额外空间来解决此问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,1,3,2,5]
 * 输出：[3,5]
 * 解释：[5, 3] 也是有效的答案。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [-1,0]
 * 输出：[-1,0]
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [0,1]
 * 输出：[1,0]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= nums.length <= 3 * 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 * 除两个只出现一次的整数外，nums 中的其他数字都出现两次
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 位运算
    /// 1. a ^ a  = 0
    ///    a ^ b != 0   ( a != b )
    /// 2. nums.xor() = a ^ b = x,  (a, b为nus中的只出现一次的数字);
    /// 3. mask = x & -x (取x的最右一位1), 此时a,b在该位必定有一个为0, 一个为1;
    /// 4. nums[i] & mask 将 nums分为两个部分, 此时a, b将分别在这两个子集合中;
    /// 5. 分别计算这两个集合的 xor_sum, 结果分别为a, b
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_sum = nums.iter().fold(0, |acc, &n| acc ^ n);
        let mask = xor_sum & -xor_sum;
        let (nums1, nums2): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&n| n & mask == 0);

        vec![
            nums1.iter().fold(0, |a, &n| a ^ n),
            nums2.iter().fold(0, |a, &n| a ^ n),
        ]
    }
}
// @lc code=end
