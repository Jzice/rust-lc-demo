/*!
 * # [1400.构造K个回文字符串](https://leetcode.cn/problems/construct-k-palindrome-strings/description/)
 *
 * @lc app=leetcode.cn id=1400 lang=rust
 *
 * ## 难度
 *
 * - Medium (57.71%)
 * - Likes:    16
 * - Dislikes: 0
 * - Total Accepted:    3.4K
 * - Total Submissions: 5.8K
 * - Testcase Example:  '"annabelle"\n2'
 *
 * ## 问题描述
 *
 * 给你一个字符串 s 和一个整数 k 。请你用 s 字符串中 所有字符 构造 k 个非空 回文串 。
 *
 * 如果你可以用 s 中所有字符构造 k 个回文字符串，那么请你返回 True ，否则返回 False 。
 *
 *
 * ## 示例 1：
 *
 * - 输入：s = "annabelle", k = 2
 * - 输出：true
 * - 解释：可以用 s 中所有字符构造 2 个回文字符串。
 * 一些可行的构造方案包括："anna" + "elble"，"anbna" + "elle"，"anellena" + "b"
 *
 *
 * ## 示例 2：
 *
 * - 输入：s = "leetcode", k = 3
 * - 输出：false
 * - 解释：无法用 s 中所有字符构造 3 个回文串。
 *
 *
 * ## 示例 3：
 *
 * - 输入：s = "true", k = 4
 * - 输出：true
 * - 解释：唯一可行的方案是让 s 中每个字符单独构成一个字符串。
 *
 *
 * ## 示例 4：
 *
 * - 输入：s = "yzyzyzyzyzyzyzy", k = 2
 * - 输出：true
 * - 解释：你只需要将所有的 z 放在一个字符串中，所有的 y 放在另一个字符串中。那么两个字符串都是回文串。
 *
 *
 * ## 示例 5：
 *
 * - 输入：s = "cr", k = 7
 * - 输出：false
 * - 解释：我们没有足够的字符去构造 7 个回文串。
 *
 *
 * 提示：
 *
 * - 1 <= s.length <= 10^5
 * - s 中所有字符都是小写英文字母。
 * - 1 <= k <= 10^5
 *
 *
 */

use super::*;

// @lc code=start

use std::collections::HashMap;
impl Solution {
    /// ## 解题思路
    /// * 回文字符串的个数取决于奇数字符的个数和k的关系
    /// * 当在k<字符串长度，且奇数字符个数>k时，
    /// * 根据抽屉原理可知，必定存在一个短串中的奇数字符个数>1, 则该短串将无法成为回文串；
    /// * 所以可以使用一个hash map统计每个字符的次数，根据map中奇数字符的次数得到正确的结果；
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut chars = HashMap::new();
        for c in s.chars() {
            let old = chars.get(&c).unwrap_or(&0);
            chars.insert(c, old + 1);
        }

        let mut ones = 0; //奇数个字符的次数
        for (_, i) in chars.into_iter() {
            ones += i & 1;
        }

        //奇数个字符数大于k，则
        return ones <= k && k <= s.len() as i32;
    }
}
// @lc code=end
