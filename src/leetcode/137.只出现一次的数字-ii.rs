/*!
 * @lc app=leetcode.cn id=137 lang=rust
 *
 * # [137] 只出现一次的数字 II
 *
 * https://leetcode.cn/problems/single-number-ii/description/
 *
 * algorithms
 * Medium (71.78%)
 * Likes:    1044
 * Dislikes: 0
 * Total Accepted:    156.7K
 * Total Submissions: 218.5K
 * Testcase Example:  '[2,2,3,2]'
 *
 * 给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。
 *
 * 你必须设计并实现线性时间复杂度的算法且不使用额外空间来解决此问题。
 *
 *
 *
 * ## 示例 1：
 *
 *
 * 输入：nums = [2,2,3,2]
 * 输出：3
 *
 *
 * ## 示例 2：
 *
 *
 * 输入：nums = [0,1,0,1,0,1,99]
 * 输出：99
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 * nums 中，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次
 *
 *
 */
struct Solution;
// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 位运算
    /// 1. 设nums的唯一一次元素为ans;
    /// 2. 各数第i位二进制位均为0,1, 如果先将其他数第i位相加,
    ///    由于这些数都出现了3次, 所以相加后结果必定为3的倍数;
    /// 3. 最后加上ans的第i位, 则结果 %3 的余数和ans的第i位一样;
    /// 4. 因此分别计算各位的和后%3, 如果不为0, 则置1;
    pub fn single_number0(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            // 计算第i位的bit和
            if nums.iter().map(|&n| (n >> i) & 1_i32).sum::<i32>() % 3 != 0 {
                // 如果第i位的bit和不为3的倍数, 则该位为1
                res |= 1 << i;
            }
        }

        res
    }

    /// - 位运算2
    /// 1. 在上面基础上优化, 同时计算所有位数;
    /// 2. 用(ai,bi)表示第i位累加和 mod 3 后的结果;
    /// 3. (ai,bi) 总共有3总情况:(00), (01), (10)
    /// 4. (ai,bi) ^ ni
    ///     (ai, bi) ^ ni     (ai', bi')
    ///     (0, 0)     0   -> (0,   0)
    ///     (0, 0)     1   -> (0,   1)
    ///     (0, 1)     0   -> (0,   1)
    ///     (0, 1)     1   -> (1,   0)
    ///     (1, 0)     0   -> (1,   0)
    ///     (1, 0)     1   -> (0,   0)  
    /// 5. ai' = (!ai &  bi & ni) | ( ai & !bi & !ni)
    ///    bi' = (!ai & !bi & ni) | (!ai &  bi & !ni) = !ai & (bi ^ ni)
    /// 6. 最后结果取b
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(a, b), &n| (a ^ n & !b, a & n | b & !n))
            .0
    }

    /// 计算a, b, 改为交替计算
    pub fn single_number3(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for n in nums {
            a = !b & (a ^ n);
            b = !a & (b ^ n);
        }
        a
    }
}
// @lc code=end
