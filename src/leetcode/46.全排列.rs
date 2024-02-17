/*!
 *
 * # [46.全排列](https://leetcode.cn/problems/permutations/description/)
 *
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * ## 难度
 * - Medium (77.19%)
 * - Likes:    1020
 * - Dislikes: 0
 * - Total Accepted:    222.9K
 * - Total Submissions: 288.6K
 * - Testcase Example:  '[1,2,3]'
 *
 * ## 问题描述
 *
 * 给定一个 没有重复 数字的序列，返回其所有可能的全排列。
 *
 * ## 示例:
 * - 输入: [1,2,3]
 * - 输出:
 * ```text
 * [
 *  [1,2,3],
 *  [1,3,2],
 *  [2,1,3],
 *  [2,3,1],
 *  [3,1,2],
 *  [3,2,1]
 * ]
 * ```
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 全排列
    /// - 回溯法
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        /// 回溯
        fn _permute(nums: &[i32], tmp: &[i32], res: &mut Vec<Vec<i32>>) {
            // 终止条件
            if nums.is_empty() {
                res.push(tmp.to_vec());
                return;
            }

            // 依次取出nums中的元素
            for (i, &val) in nums.iter().enumerate() {
                let (mut nums_v, mut tmp_v) = (nums.to_vec(), tmp.to_vec());
                nums_v.remove(i);   // 取出第i个元素
                tmp_v.push(val);    // 将第i个元素加入到临时数组中
                                    //
                // 
                _permute(&nums_v, &tmp_v, res);
            }
        }

        _permute(&nums, &vec![], &mut res);

        res
    }
}
// @lc code=end
