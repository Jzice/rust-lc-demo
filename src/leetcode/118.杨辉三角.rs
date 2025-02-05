/*!
 * # [118.杨辉三角](https://leetcode.cn/problems/pascals-triangle/description/)
 *
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * ## 难度
 * - Easy (74.15%)
 * - Likes:    708
 * - Dislikes: 0
 * - Total Accepted:    271.5K
 * - Total Submissions: 365.5K
 * - Testcase Example:  '5'
 *
 * ## 描述
 *
 * 给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。
 *
 * 在「杨辉三角」中，每个数是它左上方和右上方的数的和。
 *
 *
 * ## 示例 1:
 * - 输入: numRows = 5
 * - 输出: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 *
 * ## 示例 2:
 * - 输入: numRows = 1
 * - 输出: [[1]]
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    /// ## 杨辉三角
    /// 1. 循环，根据上层vec计算当前层的vec
    /// 2. 注意iter算子：
    ///     * windows(2)
    ///     * map
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1_i32]];
        for i in 1..num_rows {
            let row: Vec<i32> = res
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[0] + x[1])
                .collect();
            res.push([&[1], &row[..], &[1]].concat());
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        )
    }
}
