/*!
 * # [695.岛屿的最大面积](https://leetcode.cn/problems/max-area-of-island/description/)
 *
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * ## 难度
 *
 * - Medium (67.92%)
 * - Total Accepted:    249K
 * - Total Submissions: 366.6K
 * - Testcase Example:  '[[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]'
 *
 * ## 问题描述
 *
 * 给你一个大小为 m x n 的二进制矩阵 grid 。
 *
 * 岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在 水平或者竖直的四个方向上 相邻。你可以假设 grid
 * 的四个边缘都被 0（代表水）包围着。
 *
 * 岛屿的面积是岛上值为 1 的单元格的数目。
 *
 * 计算并返回 grid 中最大的岛屿面积。如果没有岛屿，则返回面积为 0 。
 *
 *
 * ## 示例 1：
 *
 * - 输入：grid =
 * [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
 * - 输出：6
 * - 解释：答案不应该是 11 ，因为岛屿只能包含水平或垂直这四个方向上的 1 。
 *
 *
 * ## 示例 2：
 *
 * - 输入：grid = [[0,0,0,0,0,0,0,0]]
 * - 输出：0
 *
 *
 * ## 提示：
 *
 * - m == grid.length
 * - n == grid[i].length
 * - 1 <= m, n <= 50
 * - grid[i][j] 为 0 或 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 深度优先搜索
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (mut stack, mut max_area) = (vec![], 0);
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut area = 0;
                stack.push((x, y));
                while let Some((x, y)) = stack.pop() {
                    if grid[y][x] != 0 && !visited[y][x] {
                        area += 1;
                        visited[y][x] = true;
                        [(1, 0), (-1, 0), (0, 1), (0, -1)]
                            .iter()
                            .map(|(dx, dy)| {
                                ((x as isize + dx) as usize, (y as isize + dy) as usize)
                            })
                            .filter(|&(x, y)| {
                                y < grid.len()
                                    && x < grid[y].len()
                                    && grid[y][x] != 0
                                    && !visited[y][x]
                            })
                            .for_each(|(x, y)| stack.push((x, y)));
                    }
                }
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}
// @lc code=end
