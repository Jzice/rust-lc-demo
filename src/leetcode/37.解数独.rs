/*!
 * # [37.解数独](https://leetcode.cn/problems/sudoku-solver/description/)
 * 
 * @lc app=leetcode.cn id=37 lang=rust
 *
 * algorithms
 * Hard (67.05%)
 * Likes:    1056
 * Dislikes: 0
 * Total Accepted:    112K
 * Total Submissions: 167K
 * Testcase Example:  '[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]'
 *
 * 编写一个程序，通过填充空格来解决数独问题。
 *
 * 数独的解法需 遵循如下规则：
 *
 *
 * 数字 1-9 在每一行只能出现一次。
 * 数字 1-9 在每一列只能出现一次。
 * 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
 *
 *
 * 数独部分空格内已填入了数字，空白格用 '.' 表示。
 *
 *
 * 示例：
 *
 *
 * 输入：board =
 * [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 *
 * 输出：[["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * 解释：输入的数独如上图所示，唯一有效的解决方案如下所示：
 *
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * board.length == 9
 * board[i].length == 9
 * board[i][j] 是一位数字或者 '.'
 * 题目数据 保证 输入数独仅有一个解
 *
 *
 *
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法
    /// 
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut col: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut mini: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        for i in 0..board.len() {
            for j in 0..board.len() {
                if board[i][j] == '.' {
                    continue;
                }
                let num = ((board[i][j] as u32) - ('1' as u32)) as usize;
                row[i][num] = true;
                col[j][num] = true;
                let k = ((i / 3) + j % 3) as usize;
                mini[k][num] = true;
            }
        }

        return Self::fill(board, 0, &mut row, &mut col, &mut mini);
    }

    fn fill(
        board: &mut Vec<Vec<char>>,
        n: u32,
        row: &mut Vec<Vec<bool>>,
        col: &mut Vec<Vec<bool>>,
        mini: &mut Vec<Vec<bool>>,
    ) {
        let n = n as usize;
        if n >= 9 {
            return;
        }
        for i in 0..board.len() {
            for j in 0..board.len() {
                if board[i][j] == '.' {
                    //
                    let k = ((i / 3) + j % 3) as usize;
                    if n <= 9 && !row[i][n] && !col[j][n] && !mini[k][n] {
                        board[i][j] = char::from_u32(n as u32).unwrap();
                        Self::fill(board, (n + 1) as u32, row, col, mini);
                        board[i][j] = '.';
                    }
                }
            }
        }
    }
}
// @lc code=end
