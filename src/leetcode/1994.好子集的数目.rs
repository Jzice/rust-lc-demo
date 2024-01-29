/*!
 * @lc app=leetcode.cn id=1994 lang=rust
 *
 * # [1994] 好子集的数目
 *
 * https://leetcode.cn/problems/the-number-of-good-subsets/description/
 *
 * ## algorithms
 * - Hard (56.69%)
 * - Likes:    133
 * - Dislikes: 0
 * - Total Accepted:    15.9K
 * - Total Submissions: 28K
 * - Testcase Example:  '[1,2,3,4]'
 *
 * 给你一个整数数组 nums 。如果 nums 的一个子集中，所有元素的乘积可以表示为一个或多个 互不相同的质数 的乘积，那么我们称它为 好子集
 * 。
 * 
 * 
 * 比方说，如果 nums = [1, 2, 3, 4] ：
 * 
 * 
 * [2, 3] ，[1, 2, 3] 和 [1, 3] 是 好 子集，乘积分别为 6 = 2*3 ，6 = 2*3 和 3 = 3 。
 * [1, 4] 和 [4] 不是 好 子集，因为乘积分别为 4 = 2*2 和 4 = 2*2 。
 * 
 * 
 * 
 * 
 * 请你返回 nums 中不同的 好 子集的数目对 10^9 + 7 取余 的结果。
 * 
 * nums 中的 子集 是通过删除 nums
 * 中一些（可能一个都不删除，也可能全部都删除）元素后剩余元素组成的数组。如果两个子集删除的下标不同，那么它们被视为不同的子集。
 * 
 * 
 * 
 * ## 示例 1：
 * 
 * 
 * - 输入：nums = [1,2,3,4]
 * - 输出：6
 * - 解释：好子集为：
 *   - [1,2]：乘积为 2 ，可以表示为质数 2 的乘积。
 *   - [1,2,3]：乘积为 6 ，可以表示为互不相同的质数 2 和 3 的乘积。
 *   - [1,3]：乘积为 3 ，可以表示为质数 3 的乘积。
 *   - [2]：乘积为 2 ，可以表示为质数 2 的乘积。
 *   - [2,3]：乘积为 6 ，可以表示为互不相同的质数 2 和 3 的乘积。
 *   - [3]：乘积为 3 ，可以表示为质数 3 的乘积。
 * 
 * 
 * ## 示例 2：
 * 
 * 
 * - 输入：nums = [4,2,3,15]
 * - 输出：5
 * - 解释：好子集为：
 *   - [2]：乘积为 2 ，可以表示为质数 2 的乘积。
 *   - [2,3]：乘积为 6 ，可以表示为互不相同质数 2 和 3 的乘积。
 *   - [2,15]：乘积为 30 ，可以表示为互不相同质数 2，3 和 5 的乘积。
 *   - [3]：乘积为 3 ，可以表示为质数 3 的乘积。
 *   - [15]：乘积为 15 ，可以表示为互不相同质数 3 和 5 的乘积。
 * 
 * 
 * 
 * 
 * ## 提示：
 * 
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 30
 * 
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        todo!();
    }
}
// @lc code=end

