/*
 * @lc app=leetcode.cn id=315 lang=rust
 *
 * [315] 计算右侧小于当前元素的个数
 *
 * https://leetcode.cn/problems/count-of-smaller-numbers-after-self/description/
 *
 * algorithms
 * Hard (43.45%)
 * Likes:    1009
 * Dislikes: 0
 * Total Accepted:    85.4K
 * Total Submissions: 196.5K
 * Testcase Example:  '[5,2,6,1]'
 *
 * 给你一个整数数组 nums ，按要求返回一个新数组 counts 。数组 counts 有该性质： counts[i] 的值是  nums[i]
 * 右侧小于 nums[i] 的元素的数量。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [5,2,6,1]
 * 输出：[2,1,1,0]
 * 解释：
 * 5 的右侧有 2 个更小的元素 (2 和 1)
 * 2 的右侧仅有 1 个更小的元素 (1)
 * 6 的右侧有 1 个更小的元素 (1)
 * 1 的右侧有 0 个更小的元素
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [-1]
 * 输出：[0]
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [-1,-1]
 * 输出：[0,0]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 归并
    /// 1. 假设将nums从中间分为左右两个子数组: left_nums, right_nums, 且都按降序排列;
    /// 2. 从左至右依次比较left_nums[i] 和 right_nums[j];
    /// 3. 如果left_nums[i] > right_nums[j], 则right_nums[j..]后的所有数字均比left_nums[i]小,
    ///    那么left_nums[i]右侧小于其的元素个数可以增加 ( right_nums.len() - j);
    /// 4. 往下递归划分,直到nums无法划分为止, 累加每个nums的上述计数, 则为结果数组;
    /// 5. 以上过程刚好是一个归并排序的过程;
    /// 6. 由于归并过程中各元素的位置会调整,因此增加一个索引数组`index[]`, 用于保存每次调整
    ///    元素位置时下标的调整情况;
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        /// 归并排序并计算
        fn merge_sort_count(
            nums: &mut [i32],
            l: usize,
            r: usize,
            index: &mut Vec<usize>,
            res: &mut Vec<i32>,
        ) {
            if l >= r - 1 {
                return;
            }

            let mid = (l + r) >> 1;
            merge_sort_count(nums, l, mid, index, res);
            merge_sort_count(nums, mid, r, index, res);

            let left_nums = nums[l..mid].to_vec();
            let right_nums = nums[mid..r].to_vec();
            let (mut ii, mut jj) = (0, 0);
            let mut i = l;
            let tmp_index = index[l..r].to_vec();
            while ii < left_nums.len() && jj < right_nums.len() {
                if left_nums[ii] > right_nums[jj] {
                    res[tmp_index[ii]] += (right_nums.len() - jj) as i32;
                    nums[i] = left_nums[ii];
                    index[i] = tmp_index[ii];
                    ii += 1;
                } else {
                    nums[i] = right_nums[jj];
                    index[i] = tmp_index[mid + jj - l];
                    jj += 1;
                }
                i += 1;
            }
            while ii < left_nums.len() {
                nums[i] = left_nums[ii];
                index[i] = tmp_index[ii];
                ii += 1;
                i += 1;
            }
            while jj < right_nums.len() {
                nums[i] = right_nums[jj];
                index[i] = tmp_index[mid + jj - l];
                jj += 1;
                i += 1;
            }
        }

        let n = nums.len();
        let mut index = (0..n).collect::<Vec<_>>();
        let mut nums = nums;
        let mut res = vec![0; n];
        merge_sort_count(&mut nums, 0, n, &mut index, &mut res);

        res
    }
}
// @lc code=end
