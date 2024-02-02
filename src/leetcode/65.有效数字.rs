/*!
 * # [65.有效数字](https://leetcode.cn/problems/valid-number/description/)
 *
 * @lc app=leetcode.cn id=65 lang=rust
 *
 * ## 难度
 * - Hard (27.58%)
 * - Likes:    352
 * - Dislikes: 0
 * - Total Accepted:    64.8K
 * - Total Submissions: 235K
 * - Testcase Example:  '"0"'
 *
 * ## 问题描述
 *
 * 有效数字（按顺序）可以分成以下几个部分：
 * - 一个 小数 或者 整数
 * - （可选）一个 'e' 或 'E' ，后面跟着一个 整数
 *
 *
 * 小数（按顺序）可以分成以下几个部分：
 * - （可选）一个符号字符（'+' 或 '-'）
 * - 下述格式之一：
 *   - 至少一位数字，后面跟着一个点 '.'
 *   - 至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字
 *   - 一个点 '.' ，后面跟着至少一位数字
 *
 * 整数（按顺序）可以分成以下几个部分：
 * - （可选）一个符号字符（'+' 或 '-'）
 * - 至少一位数字
 *
 * 部分有效数字列举如下：["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3",
 * "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
 *
 * 部分无效数字列举如下：["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
 *
 * 给你一个字符串 s ，如果 s 是一个 有效数字 ，请返回 true 。
 *
 *
 * ## 示例 1：
 * - 输入：s = "0"
 * - 输出：true
 *
 * ## 示例 2：
 * - 输入：s = "e"
 * - 输出：false
 *
 * ## 示例 3：
 * - 输入：s = "."
 * - 输出：false
 *
 * ## 提示：
 * - 1 <= s.length <= 20
 * - s 仅含英文字母（大写和小写），数字（0-9），加号 '+' ，减号 '-' ，或者点 '.' 。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 有效数字
    /// ## 解题思路
    pub fn is_number(s: String) -> bool {
        fn is_decimal(s: &str) -> bool {
            let (mut num, mut dot) = (false, false);
            for (i, c) in s.chars().enumerate() {
                match c {
                    '+' | '-' if i == 0 => {}
                    '0'..='9' => num = true,
                    '.' if !dot => dot = true,
                    _ => return false,
                }
            }
            num
        }

        fn is_int(s: &str) -> bool {
            let mut num = false;
            for (i, c) in s.chars().enumerate() {
                match c {
                    '+' | '-' if i == 0 => {}
                    '0'..='9' => num = true,
                    _ => return false,
                }
            }
            num
        }

        if let Some(e) = s.chars().position(|c| c == 'e' || c == 'E') {
            is_decimal(&s[0..e]) && is_int(&s[e + 1..])
        } else {
            is_decimal(&s)
        }
    }
}
// @lc code=end
