/*!
 * # [20.有效的括号](https://leetcode.cn/problems/valid-parentheses/description/)
 *
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * ## 难度
 * - Easy (44.42%)
 * - Likes:    3638
 * - Dislikes: 0
 * - Total Accepted:    1.3M
 * - Total Submissions: 3M
 * - Testcase Example:  '"()"'
 *
 * ## 描述
 *
 * 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
 * 
 * 有效字符串需满足：
 * - 左括号必须用相同类型的右括号闭合。
 * - 左括号必须以正确的顺序闭合。
 * - 每个右括号都有一个对应的相同类型的左括号。
 * 
 * ## 示例 1：
 * - 输入：s = "()"
 * - 输出：true
 * 
 * ## 示例 2：
 * - 输入：s = "()[]{}"
 * - 输出：true
 * 
 * ## 示例 3：
 * - 输入：s = "(]"
 * - 输出：false
 * 
 * 
 * ## 提示：
 * - 1 <= s.length <= 10^4
 * - s 仅由括号 '()[]{}' 组成
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 有效的括号
    /// - 栈+hashmap
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let mut tmp = vec![];
        let pairs = HashMap::from([
            (')', '('),
            ('}', '{'),
            (']', '['),
        ]);

        for c in s.chars() {
            match c {
                '('|'['|'{' => {
                    tmp.push(c);
                },
                ')'|']'|'}' => {
                    if tmp.last() != pairs.get(&c) {
                        return false;
                    } else {
                        tmp.pop();
                    }
                },   
                _ => {}
            }
        }
        return tmp.len() == 0;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid("(]".into()), false);
    }
}
