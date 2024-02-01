/*!
 * # [1662.检查两个字符串数组是否相等](https://leetcode.cn/problems/check-if-two-string-arrays-are-equivalent/description/)
 *
 * @lc app=leetcode.cn id=1662 lang=rust 
 *
 * ## 难度
 *
 * - Easy (80.93%)
 *
 * ## 问题描述
 *
 * 给你两个字符串数组 word1 和 word2 。如果两个数组表示的字符串相同，返回 true ；否则，返回 false 。
 * 数组表示的字符串 是由数组中的所有元素 按顺序 连接形成的字符串。
 *  
 * ## 示例 1：
 *
 * - 输入：word1 = ["ab", "c"], word2 = ["a", "bc"]
 * - 输出：true
 * - 解释：
 *   - word1 表示的字符串为 "ab" + "c" -> "abc"
 *   - word2 表示的字符串为 "a" + "bc" -> "abc"
 *   - 两个字符串相同，返回 true
 *
 *
 * ## 示例 2：
 *
 * - 输入：word1 = ["a", "cb"], word2 = ["ab", "c"]
 * - 输出：false
 *
 * ## 示例 3：
 *
 * - 输入：word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
 * - 输出：true
 *  
 * ## 提示：
 * 
 * - word1[i] 和 word2[i] 由小写字母组成
 *
 * ## 测试用例:
 * ```text
 *   ["ab", "c"]
 *   ["a", "bc"]
 * ```
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 1662.检查两个字符串数组是否相等
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.into_iter().fold(String::new(), |acc, w| format!("{}{}", acc, w))
            == word2.into_iter().fold(String::new(), |acc, w| format!("{}{}", acc, w))
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
            Solution::array_strings_are_equal(
                vec!["ab".into(), "c".into()], 
                vec!["a".into(), "bc".into()]
            ), 
            true
        );
    }
}
        
