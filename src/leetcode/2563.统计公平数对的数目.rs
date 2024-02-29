/*!
 * # [2563.统计公平数对的数目](https://leetcode.cn/problems/count-the-number-of-fair-pairs/description/)
 *
 * @lc app=leetcode.cn id=2563 lang=rust
 *
 * ## 难度
 * - Medium (34.06%)
 * - Likes:    38
 * - Dislikes: 0
 * - Total Accepted:    7.5K
 * - Total Submissions: 21.9K
 * - Testcase Example:  '[0,1,7,4,4,5]\n3\n6'
 *
 * ## 描述
 *
 * 给你一个下标从 0 开始、长度为 n 的整数数组 nums ，和两个整数 lower 和 upper ，返回 公平数对的数目 。
 *
 * 如果 (i, j) 数对满足以下情况，则认为它是一个 公平数对 ：
 *
 * - 0 <= i < j < n，且
 * - lower <= nums[i] + nums[j] <= upper
 *
 * ## 示例 1：
 * - 输入：nums = [0,1,7,4,4,5], lower = 3, upper = 6
 * - 输出：6
 * - 解释：共计 6 个公平数对：(0,3)、(0,4)、(0,5)、(1,3)、(1,4) 和 (1,5) 。
 *
 *
 * ## 示例 2：
 * - 输入：nums = [1,7,9,2,5], lower = 11, upper = 11
 * - 输出：1
 * - 解释：只有单个公平数对：(2,3) 。
 *
 * ## 提示：
 * - 1 <= nums.length <= 10^5
 * - nums.length == n
 * - -10^9 <= nums[i] <= 10^9
 * - -10^9 <= lower <= upper <= 10^9
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 双指针
    /// 1. count(lower <= nums[i]+nums[j] <= upper) 等价于
    ///    count(nums[i]+nums[j] <= upper) - count(nums[i]+nums[j] <= lower)
    /// 2. 计算count可使用双指针法:
    /// 3. 先将nums从小到大排序;
    /// 4. 然后设置双指针(l, r)分别指向nums左右两端;
    /// 5. 依次判断nums[l]+nums[r], 如果>upper, 则r左移(r-=1);
    /// 6. 否则count+=1, 并且l右移(l+=1);
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        /// 计算
        fn count(nums: &[i32], upper: i32) -> i64 {
            let (mut l, mut r) = (0, nums.len() - 1);
            let mut res = 0;
            while l < r {
                if (nums[l] as i64) + (nums[r] as i64) <= upper as i64 {
                    res += (r - l) as i64;
                    l += 1;
                } else {
                    r -= 1;
                }
            }

            res
        }

        let mut nums = nums;
        nums.sort();

        count(&nums, upper) - count(&nums, lower - 1)
    }
}
// @lc code=end
