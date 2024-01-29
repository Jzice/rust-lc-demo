/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 *
 * https://leetcode.cn/problems/number-of-islands/description/
 *
 * algorithms
 * Medium (59.41%)
 * Likes:    2265
 * Dislikes: 0
 * Total Accepted:    686.3K
 * Total Submissions: 1.2M
 * Testcase Example:  '[["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]'
 *
 * 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
 *
 * 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
 *
 * 此外，你可以假设该网格的四条边均被水包围。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：grid = [
 * ⁠ ["1","1","1","1","0"],
 * ⁠ ["1","1","0","1","0"],
 * ⁠ ["1","1","0","0","0"],
 * ⁠ ["0","0","0","0","0"]
 * ]
 * 输出：1
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [
 * ⁠ ["1","1","0","0","0"],
 * ⁠ ["1","1","0","0","0"],
 * ⁠ ["0","0","1","0","0"],
 * ⁠ ["0","0","0","1","1"]
 * ]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1
 * grid[i][j] 的值为 '0' 或 '1'
 *
 *
 */

struct Solution;

// @lc code=start

struct UnionFindSet {
    n: usize,
    pa: Vec<usize>,
}

impl UnionFindSet {
    pub fn new(n: usize) -> Self {
        UnionFindSet {
            n,
            pa: (0..n).collect::<Vec<_>>(),
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn find(&mut self, a: usize) -> usize {
        let mut a_ = a;
        while self.pa[a_] != a_ {
            a_ = self.pa[a_];
        }
        self.pa[a] = a_;

        a_
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let (a_, b_) = (self.find(a), self.find(b));
        if a_ != b_ {
            self.pa[a_] = b_;
            self.n -= 1;
        }
    }
}

impl Solution {
    /// ## 解题思路
    /// - 并查集
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut water = 0_i32;
        let mut ufs = UnionFindSet::new(m * n);

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '0' {
                    water += 1;
                    continue;
                }

                [(0, 1), (0, !0), (1, 0), (!0, 0)]
                    .iter()
                    .map(|&d| (i.wrapping_add(d.0), j.wrapping_add(d.1)))
                    .filter(|(di, dj)| (0..m).contains(di) && (0..n).contains(dj))
                    .filter(|(di, dj)| grid[*di][*dj] == '1')
                    .for_each(|(di, dj)| {
                        ufs.union(di * n + dj, i * n + j);
                    });
            }
        }

        ufs.size() as i32 - water
    }
}
// @lc code=end
