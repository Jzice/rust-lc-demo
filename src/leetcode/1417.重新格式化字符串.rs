/*!
 * # [1417.重新格式化字符串]( https://leetcode.cn/problems/reformat-the-string/description/)
 *
 * @lc app=leetcode.cn id=1417 lang=rust
 *
 * ## 难度
 *
 * - Easy (55.17%)
 * - Likes:    92
 * - Dislikes: 0
 * - Total Accepted:    46.8K
 * - Total Submissions: 84.7K
 * - Testcase Example:  '"a0b1c2"'
 *
 *
 * ## 问题描述
 *
 * 给你一个混合了数字和字母的字符串 s，其中的字母均为小写英文字母。
 *
 * 请你将该字符串重新格式化，使得任意两个相邻字符的类型都不同。也就是说，字母后面应该跟着数字，而数字后面应该跟着字母。
 *
 * 请你返回 重新格式化后 的字符串；如果无法按要求重新格式化，则返回一个 空字符串 。
 *
 *
 * ## 示例 1：
 *
 * 输入：s = "a0b1c2"
 * 输出："0a1b2c"
 * 解释："0a1b2c" 中任意两个相邻字符的类型都不同。 "a0b1c2", "0a1b2c", "0c2a1b" 也是满足题目要求的答案。
 *
 *
 * ## 示例 2：
 *
 * 输入：s = "leetcode"
 * 输出：""
 * 解释："leetcode" 中只有字母，所以无法满足重新格式化的条件。
 *
 *
 * ## 示例 3：
 *
 * 输入：s = "1229857369"
 * 输出：""
 * 解释："1229857369" 中只有数字，所以无法满足重新格式化的条件。
 *
 *
 * ## 示例 4：
 *
 * 输入：s = "covid2019"
 * 输出："c2o0v1i9d"
 *
 *
 * ## 示例 5：
 *
 * 输入：s = "ab123"
 * 输出："1a2b3"
 *
 *
 * ## 提示：
 *
 * - 1 <= s.length <= 500
 * - s 仅由小写英文字母和/或数字组成。
 *
 */

// @lc code=start
impl Solution {
    /// # 重新格式化字符串
    /// ## 解题思路
    pub fn reformat(s: String) -> String {
        // 统计字母和数字的个数
        let char_count = s.chars().filter(|c| c.is_alphabetic()).count() as i32;
        let digit_count = (s.len() as i32) - char_count;
        // 如果字母和数字个数相差>1, 则无法格式化, 返回""
        if (char_count - digit_count).abs() > 1 {
            return "".to_owned();
        }
        // 个数多的字符放开头
        let (mut i, mut j) = match char_count >= digit_count {
            true => (0, 1),
            false => (1, 0),
        };

        // 依次将各个字符交替放到目标字符数组中
        let mut res_vec = vec![' '; s.len()];
        for c in s.chars() {
            match c.is_ascii_alphabetic() {
                true => {
                    res_vec[i] = c;
                    i += 2;
                }
                false => {
                    res_vec[j] = c;
                    j += 2;
                }
            }
        }

        // 将字符数组转换成字符串
        res_vec.into_iter().collect()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::reformat("a0b1c2".to_string()), "a0b1c2".to_string());
        assert_eq!(Solution::reformat("leetcode".to_string()), "".to_string());
    }

}
