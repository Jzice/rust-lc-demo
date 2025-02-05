/*!
 *
 * # [28.实现 strStr()](https://leetcode.cn/problems/implement-strstr/description/)
 *
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * ## 难度
 * - Easy (40.37%)
 * - Likes:    1155
 * - Dislikes: 0
 * - Total Accepted:    515.5K
 * - Total Submissions: 1.3M
 * - Testcase Example:  '"hello"\n"ll"'
 *
 * ## 描述
 *
 * 实现 strStr() 函数。
 * 
 * 给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串出现的第一个位置（下标从 0
 * 开始）。如果不存在，则返回  -1 。
 * 
 * 
 * 
 * 说明：
 * 
 * 当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
 * 
 * 对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与 C 语言的 strstr() 以及 Java 的 indexOf()
 * 定义相符。
 * 
 * 
 * 
 * ## 示例 1：
 * 
 * 
 * 输入：haystack = "hello", needle = "ll"
 * 输出：2
 * 
 * 
 * ## 示例 2：
 * - 输入：haystack = "aaaaa", needle = "bba"
 * - 输出：-1
 * 
 * 
 * ## 示例 3：
 * - 输入：haystack = "", needle = ""
 * - 输出：0
 * 
 * 
 * ## 提示：
 * - haystack 和 needle 仅由小写英文字符组成
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let size = needle.len();
        let needle = needle.as_bytes();
        if size == 0 { return 0i32 }
        match haystack.as_bytes()
            .windows(size)
            .position(|s| s == needle) {
                Some(x) => x as i32,
                _ => -1,
            }
    }
}
// @lc code=end

