/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 *
 * https://leetcode.cn/problems/word-search/description/
 *
 * algorithms
 * Medium (46.26%)
 * Likes:    1620
 * Dislikes: 0
 * Total Accepted:    434.3K
 * Total Submissions: 938.9K
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false
 * 。
 *
 * 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word =
 * "ABCCED"
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word =
 * "SEE"
 * 输出：true
 *
 *
 * 示例 3：
 *
 *
 * 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word =
 * "ABCB"
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == board.length
 * n = board[i].length
 * 1
 * 1
 * board 和 word 仅由大小写英文字母组成
 *
 *
 *
 *
 * 进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - dfs
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        // 深度优先搜索
        fn dfs(
            board: &Vec<Vec<char>>,           // 棋盘
            word: &Vec<char>,                 // 字符数组
            visited: &mut Vec<Vec<bool>>,     // 回溯标记
            (i, r, c): (usize, usize, usize), // 遍历标记
        ) -> bool {
            // 边界条件, 剪枝
            if r >= visited.len()
                || c >= visited[0].len()
                || visited[r][c]
                || word[i] != board[r][c]
            {
                return false;
            }

            // 终止条件
            if i == word.len() - 1 {
                return true;
            }

            // 标记遍历状态
            visited[r][c] = true;
            // 尝试下一步搜索
            if dfs(board, word, visited, (i + 1, r + 1, c))
                || dfs(board, word, visited, (i + 1, r - 1, c))
                || dfs(board, word, visited, (i + 1, r, c + 1))
                || dfs(board, word, visited, (i + 1, r, c - 1))
            {
                return true;
            }
            // 恢复遍历状态
            visited[r][c] = false;

            return false;
        }

        let word = word.chars().collect::<Vec<_>>();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if dfs(&board, &word, &mut visited, (0, r, c)) {
                    return true;
                }
            }
        }

        false
    }
}

// @lc code=end

struct Solution;
