/*!
 * # [6.Z字形变换](https://leetcode.cn/problems/zigzag-conversion/description/)
 *
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * ## 难度
 * - Medium (45.86%)
 * - Likes:    564
 * - Dislikes: 0
 * - Total Accepted:    95.3K
 * - Total Submissions: 204.1K
 * - Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * ## 问题描述
 * 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
 *
 * 比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
 *
 * ```text
 * L   C   I   R
 * E T O E S I I G
 * E   D   H   N
 * ``` 
 *
 *
 * 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
 *
 * 请你实现这个将字符串进行指定行数变换的函数：
 *
 * string convert(string s, int numRows);
 *
 * ## 示例 1:
 * - 输入: s = "LEETCODEISHIRING", numRows = 3
 * - 输出: "LCIRETOESIIGEDHN"
 *
 *
 * ## 示例 2:
 * - 输入: s = "LEETCODEISHIRING", numRows = 4
 * - 输出: "LDREOEIIECIHNTSG"
 * - 解释:
 * ```text
 * L     D     R
 * E   O E   I I
 * E C   I H   N
 * T     S     G
 * ``` 
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    /// # Z字形变换
    /// ## 解题思路
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = num_rows;
        let it = (0..n).into_iter().chain((1..n - 1).rev().into_iter());
        let mut vr: Vec<Vec<char>> = vec![vec![]; n as usize];
        s.chars()
            .zip(it.cycle())
            .for_each(|(c, i)| vr[i as usize].push(c));

        vr.into_iter().flatten().collect()
    }
}
// @lc code=end
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::convert("LEETCODEISHIRING".into(), 4),
            "LDREOEIIECIHNTSG".to_string()
        );
    }
}
