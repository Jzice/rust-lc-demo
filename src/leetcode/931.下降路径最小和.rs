/*!
 * # [931.下降路径最小和](https://leetcode.cn/problems/minimum-falling-path-sum/description/)
 *
 * @lc app=leetcode.cn id=931 lang=rust
 *
 * ## 难度
 * - Medium (67.61%)
 * - Likes:    337
 * - Dislikes: 0
 * - Total Accepted:    98.7K
 * - Total Submissions: 146K
 * - Testcase Example:  '[[2,1,3],[6,5,4],[7,8,9]]'
 *
 * ## 描述
 *
 * 给你一个 n x n 的 方形 整数数组 matrix ，请你找出并返回通过 matrix 的下降路径 的 最小和 。
 * 
 * 下降路径
 * 可以从第一行中的任何元素开始，并从每一行中选择一个元素。在下一行选择的元素和当前行所选元素最多相隔一列（即位于正下方或者沿对角线向左或者向右的第一个元素）。具体来说，位置
 * (row, col) 的下一个元素应当是 (row + 1, col - 1)、(row + 1, col) 或者 (row + 1, col + 1)
 * 。
 * 
 * ## 示例 1：
 * - 输入：matrix = [[2,1,3],[6,5,4],[7,8,9]]
 * - 输出：13
 * - 解释：如图所示，为和最小的两条下降路径
 * 
 * ## 示例 2：
 * - 输入：matrix = [[-19,57],[-40,-5]]
 * - 输出：-59
 * - 解释：如图所示，为和最小的下降路径
 * 
 * ## 提示：
 * - n == matrix.length == matrix[i].length
 * - 1 <= n <= 100
 * - -100 <= matrix[i][j] <= 100
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 下降路径最小和
    /// ## 动态规划
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        *matrix.into_iter()
            .fold(vec![0; n], |pre, row| {
                row.iter()
                    .enumerate()
                    .map(|(i, &v)| {
                        v + pre[i]
                            .min(*pre.get(i.checked_add(1).unwrap_or(i)).unwrap_or(&i32::MAX))
                            .min(*pre.get(i.checked_sub(1).unwrap_or(i)).unwrap_or(&i32::MAX))
                    })
                    .collect() 
            })
            .iter()
            .min()
            .unwrap_or(&0)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::min_falling_path_sum(vec![
                vec![2,1,3],
                vec![6,5,4],
                vec![7,8,9]
        ]), 13);
        assert_eq!(Solution::min_falling_path_sum(vec![
                vec![-19,57],
                vec![-40,-5]
        ]), -59);
        assert_eq!(Solution::min_falling_path_sum(vec![
                vec![-48],
        ]), -48);
    }

}
