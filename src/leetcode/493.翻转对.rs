/*
 * @lc app=leetcode.cn id=493 lang=rust
 *
 * [493] 翻转对
 *
 * https://leetcode.cn/problems/reverse-pairs/description/
 *
 * algorithms
 * Hard (36.60%)
 * Likes:    418
 * Dislikes: 0
 * Total Accepted:    41.9K
 * Total Submissions: 114.3K
 * Testcase Example:  '[1,3,2,3,1]'
 *
 * 给定一个数组 nums ，如果 i < j 且 nums[i] > 2*nums[j] 我们就将 (i, j) 称作一个重要翻转对。
 *
 * 你需要返回给定数组中的重要翻转对的数量。
 *
 * 示例 1:
 *
 *
 * 输入: [1,3,2,3,1]
 * 输出: 2
 *
 *
 * 示例 2:
 *
 *
 * 输入: [2,4,3,5,1]
 * 输出: 3
 *
 *
 * 注意:
 *
 *
 * 给定数组的长度不会超过50000。
 * 输入数组中的所有数字都在32位整数的表示范围内。
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 归并排序
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        fn merge_sort_count(nums: &mut [i32], left: usize, right: usize, count: &mut i32) {
            if left >= right - 1 {
                return;
            }

            let mid = (left + right) >> 1;
            merge_sort_count(nums, left, mid, count);
            merge_sort_count(nums, mid, right, count);

            let (mut l, mut r) = (left, mid);
            for l in left..mid {
                while r < right && (nums[l] as i64) <= (nums[r] as i64) * 2 {
                    r += 1;
                }
                if r < right && (nums[l] as i64) > (nums[r] as i64) * 2 {
                    *count += (right - r) as i32
                }
            }

            let l_nums = nums[left..mid].to_vec();
            let r_nums = nums[mid..right].to_vec();
            let (mut l, mut r) = (0, 0);
            let mut i = left;
            while l < l_nums.len() && r < r_nums.len() {
                if l_nums[l] as i64 >= r_nums[r] as i64 {
                    nums[i] = l_nums[l];
                    l += 1;
                } else {
                    nums[i] = r_nums[r];
                    r += 1;
                }
                i += 1;
            }
            while l < l_nums.len() {
                nums[i] = l_nums[l];
                l += 1;
                i += 1;
            }
            while r < r_nums.len() {
                nums[i] = r_nums[r];
                r += 1;
                i += 1;
            }
        }

        let n = nums.len();
        let mut nums = nums;
        let mut count = 0;
        merge_sort_count(&mut nums, 0, n, &mut count);
        count
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    }
}
