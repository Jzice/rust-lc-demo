/*!
 * @lc app=leetcode.cn id=327 lang=rust
 *
 * [327] 区间和的个数
 (
 https://leetcode.cn/problems/count-of-range-sum/description/
 )
 *
 * algorithms
 * Hard (40.69%)
 * Likes:    547
 * Dislikes: 0
 * Total Accepted:    41.6K
 * Total Submissions: 102.4K
 * Testcase Example:  '[-2,5,-1]\n-2\n2'
 *
 * 给你一个整数数组 nums 以及两个整数 lower 和 upper 。求数组中，值位于范围 [lower, upper] （包含 lower 和
 * upper）之内的 区间和的个数 。
 *
 * 区间和 S(i, j) 表示在 nums 中，位置从 i 到 j 的元素之和，包含 i 和 j (i ≤ j)。
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-2,5,-1], lower = -2, upper = 2
 * 输出：3
 * 解释：存在三个区间：[0,0]、[2,2] 和 [0,2] ，对应的区间和分别是：-2 、-1 、2 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0], lower = 0, upper = 0
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -2^31
 * -10^5
 * 题目数据保证答案是一个 32 位 的整数
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 前缀和+归并排序
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // 计算前缀和
        let mut prefix_sums = vec![0_i64];
        for &n in &nums {
            let last = *prefix_sums.last().unwrap_or(&0);
            prefix_sums.push(n as i64 + last);
        }

        fn merge(
            prefix_sums: &mut Vec<i64>,
            left: usize,
            right: usize,
            res: &mut i64,
            lower: i64,
            upper: i64,
        ) {
            if right - left <= 1 {
                return;
            }

            let mid = (left + right) >> 1;
            merge(prefix_sums, left, mid, res, lower, upper);
            merge(prefix_sums, mid, right, res, lower, upper);

            // 计算区间和个数
            let (mut s, mut e) = (mid, mid);
            for l in left..mid {
                while s < right && prefix_sums[s] - prefix_sums[l] < lower {
                    s += 1;
                }
                while e < right && prefix_sums[e] - prefix_sums[l] <= upper {
                    e += 1;
                }
                *res += (e - s) as i64;
            }

            // 合并
            let left_sums = prefix_sums[left..mid].to_vec();
            let right_sums = prefix_sums[mid..right].to_vec();
            let (mut l, mut r) = (0, 0);
            let mut i = left;
            while l < left_sums.len() && r < right_sums.len() {
                if left_sums[l] < right_sums[r] {
                    prefix_sums[i] = left_sums[l];
                    l += 1;
                } else {
                    prefix_sums[i] = right_sums[r];
                    r += 1;
                }
                i += 1;
            }
            while l < left_sums.len() {
                prefix_sums[i] = left_sums[l];
                l += 1;
                i += 1;
            }
            while r < right_sums.len() {
                prefix_sums[i] = right_sums[r];
                r += 1;
                i += 1;
            }
        }

        let n = prefix_sums.len();
        let mut res = 0;
        merge(&mut prefix_sums, 0, n, &mut res, lower as i64, upper as i64);
        res as i32
    }
}
// @lc code=end
