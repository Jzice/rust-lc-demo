/*!
 * # [73.矩阵置零]( https://leetcode.cn/problems/set-matrix-zeroes/description/)
 *
 * @lc app=leetcode.cn id=73 lang=rust
 *
 * ## 难度
 * - Medium (63.20%)
 * - Likes:    888
 * - Dislikes: 0
 * - Total Accepted:    256.3K
 * - Total Submissions: 405.6K
 * - Testcase Example:  '[[1,1,1],[1,0,1],[1,1,1]]'
 *
 * ## 问题描述
 * 给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。
 * 请使用 原地 算法。
 *
 * ## 示例 1：
 * - 输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
 * - 输出：[[1,0,1],[0,0,0],[1,0,1]]
 *
 * ## 示例 2：
 * - 输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 * - 输出：[[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 *
 * ## 提示：
 * - m == matrix.length
 * - n == matrix[0].length
 * - 1 <= m, n <= 200
 * - -2^31 <= matrix[i][j] <= 2^31 - 1
 *
 *
 * ## 进阶：
 * - 一个直观的解决方案是使用  O(mn) 的额外空间，但这并不是一个好的解决方案。
 * - 一个简单的改进方案是使用 O(m + n) 的额外空间，但这仍然不是最好的解决方案。
 * - 你能想出一个仅使用常量空间的解决方案吗？
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// 矩阵置零
    /// ## 解题思路
    /// 1. 第一次遍历所有元素,标记所有0元素对应的第0行,第0列元素;
    /// 2. 分别遍历第0行,第0列,将其中标记元素对应的列,行所有元素置为0;
    /// 3. 如果第0行,第0列中有0, 则将第0行,第0列也都全部置为0;
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut col_0, mut row_0) = (false, false);
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    if i == 0 {
                        col_0 = true;
                    }
                    if j == 0 {
                        row_0 = true;
                    }
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        (1..m).for_each(|i| {
            if matrix[i][0] == 0 {
                (1..n).for_each(|j| matrix[i][j] = 0);
            }
        });

        (1..n).for_each(|j| {
            if matrix[0][j] == 0 {
                (1..m).for_each(|i| matrix[i][j] = 0);
            }
        });

        if col_0 {
            (0..n).for_each(|j| matrix[0][j] = 0);
        }
        if row_0 {
            (0..m).for_each(|i| matrix[i][0] = 0);
        }
    }
}
// @lc code=end
