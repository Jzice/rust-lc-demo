/*!
 *
 * # [1818.绝对差值和] ( https://leetcode.cn/problems/minimum-absolute-sum-difference/description/)
 *
 * @lc app=leetcode.cn id=1818 lang=rust
 *
 * ## 难度
 * - Medium (37.55%)
 * - Likes:    156
 * - Dislikes: 0
 * - Total Accepted:    31.9K
 * - Total Submissions: 85K
 * - Testcase Example:  '[1,7,5]\n[2,3,5]'
 *
 * ## 描述
 *
 * 给你两个正整数数组 nums1 和 nums2 ，数组的长度都是 n 。
 *
 * 数组 nums1 和 nums2 的 绝对差值和 定义为所有 |nums1[i] - nums2[i]|（0 ）的 总和（下标从 0 开始）。
 *
 * 你可以选用 nums1 中的 任意一个 元素来替换 nums1 中的 至多 一个元素，以 最小化 绝对差值和。
 *
 * 在替换数组 nums1 中最多一个元素 之后 ，返回最小绝对差值和。因为答案可能很大，所以需要对 10^9 + 7 取余 后返回。
 *
 * |x| 定义为：
 *
 * 如果 x >= 0 ，值为 x ，或者
 * 如果 x  ，值为 -x
 *
 *
 *
 *
 * ## 示例 1：
 *
 *
 * 输入：nums1 = [1,7,5], nums2 = [2,3,5]
 * 输出：3
 * 解释：有两种可能的最优方案：
 * - 将第二个元素替换为第一个元素：[1,7,5] => [1,1,5] ，或者
 * - 将第二个元素替换为第三个元素：[1,7,5] => [1,5,5]
 * 两种方案的绝对差值和都是 |1-2| + (|1-3| 或者 |5-3|) + |5-5| = 3
 *
 *
 * ## 示例 2：
 * - 输入：nums1 = [2,4,6,8,10], nums2 = [2,4,6,8,10]
 * - 输出：0
 * - 解释：nums1 和 nums2 相等，所以不用替换元素。绝对差值和为 0
 *
 *
 * ## 示例 3：
 * - 输入：nums1 = [1,10,4,4,2,7], nums2 = [9,3,5,1,7,4]
 * - 输出：20
 * - 解释：将第一个元素替换为第二个元素：[1,10,4,4,2,7] => [10,10,4,4,2,7]
 * 绝对差值和为 |10-9| + |10-3| + |4-5| + |4-1| + |2-7| + |7-4| = 20
 *
 *
 * ## 提示：
 * - n == nums1.length
 * - n == nums2.length
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分法
    /// 1. 原绝对差和为: diff_sum = sum(|nums1[i]-nums2[i]|)(i=[0..n]),
    /// 2. 要使替换后的绝对差和最小, 则对于每一个i, |nums1[j] - nums2[i]| 尽量小,
    ///    因此需要在nums1中查找和当前nums2[i]最接近的nums1[j]
    /// 3. 为快速查找nums1中最接近nums2[i]的数, 先对nums1进行排序, 再使用二分法快速查找j;
    /// 4. 使用二分查找在排序后的sorted_nums1中查找nums2[i]. 如果找到,
    ///    否则如果没找到, 则计算最近的sorted_nums1[j], sorted_nums1[j-1]二者中最接近nums2[i]的那个;
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = 1_000_000_007;
        let n = nums1.len();
        let mut max_delta = 0;
        let mut diff_sum = 0;

        let mut sorted_nums1 = nums1.clone();
        sorted_nums1.sort();

        for i in 0..n {
            let diff = (nums1[i] - nums2[i]).abs();
            diff_sum = (diff_sum + diff) % m;
            match sorted_nums1.binary_search(&nums2[i]) {
                Ok(j) => {
                    max_delta = max_delta.max(diff - 0);
                }
                Err(j) => {
                    if j < n {
                        max_delta = max_delta.max(diff - (sorted_nums1[j] - nums2[i]));
                    }
                    if j > 0 {
                        max_delta = max_delta.max(diff - (nums2[i] - sorted_nums1[j - 1]));
                    }
                }
            }
        }

        (diff_sum - max_delta + m) % m
    }
}
// @lc code=end
