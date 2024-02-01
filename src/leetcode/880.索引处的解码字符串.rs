/*!
 * # [880.索引处的解码字符串](https://leetcode.cn/problems/decoded-string-at-index/description/)
 *
 * @lc app=leetcode.cn id=880 lang=rust
 *
 * ## 难度
 *
 * - Medium (27.00%)
 * - Likes:    181
 * - Dislikes: 0
 * - Total Accepted:    8.8K
 * - Total Submissions: 32.6K
 * - Testcase Example:  '"leet2code3"\n10'
 *
 * ## 题目描述
 *
 * 给定一个编码字符串 S。请你找出 解码字符串 并将其写入磁带。解码时，从编码字符串中 每次读取一个字符 ，并采取以下步骤：
 *
 * 如果所读的字符是字母，则将该字母写在磁带上。
 * 如果所读的字符是数字（例如 d），则整个当前磁带总共会被重复写 d-1 次。
 *
 * 现在，对于给定的编码字符串 S 和索引 K，查找并返回解码字符串中的第 K 个字母。
 *
 *
 * ## 示例 1：
 *
 * - 输入：S = "leet2code3", K = 10
 * - 输出："o"
 * - 解释：
 * - 解码后的字符串为 "leetleetcodeleetleetcodeleetleetcode"。
 * - 字符串中的第 10 个字母是 "o"。
 *
 *
 * ## 示例 2：
 *
 * - 输入：S = "ha22", K = 5
 * - 输出："h"
 * - 解释：
 * - 解码后的字符串为 "hahahaha"。第 5 个字母是 "h"。
 *
 *
 * ## 示例 3：
 *
 * - 输入：S = "a2345678999999999999999", K = 1
 * - 输出："a"
 * - 解释：
 * - 解码后的字符串为 "a" 重复 8301530446056247680 次。第 1 个字母是 "a"。
 *
 *
 * ## 提示：
 *
 * - 2 <= S.length <= 100
 * - S 只包含小写字母与数字 2 到 9 。
 * - S 以字母开头。
 * - 1 <= K <= 10^9
 * - 题目保证 K 小于或等于解码字符串的长度。
 * - 解码后的字符串保证少于 2^63 个字母。
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 索引处的解码字符串
    /// ## 解题思路
    /// 1. 编码字符串 "apple6"中的第24个元素和第4个元素相同,
    ///    即 k(24) % l(5);
    /// 2. 先正向遍历编码字符串,计算解码字符串有效长度`decode_len`, 长度只用计算到>=k时为止;
    /// 3. 从有效长度的编码字符串下标开始, 逆序遍历待解码字符串, 反向更新解码字符串长度,
    ///    此时如果为重复数字, 可根据1中的原理, 缩小k % l;
    /// 4. 如果k % 解码字符串长度==0, 则当前字符为第k个字符;
    pub fn decode_at_index(s: String, k: i32) -> String {
        let sb = s.as_bytes();
        let mut k = k as usize;
        let mut decode_len = 0; //解码字符串的长度

        // 正向遍历编码字符串,计算有效的解码字符串长度
        let mut off = 0;
        while off < sb.len() {
            let c = sb[off];
            if c.is_ascii_digit() {
                decode_len *= (c - b'0') as usize;
                if decode_len >= k {
                    break;
                }
            } else {
                decode_len += 1;
                if decode_len == k {
                    return char::from(c).to_string();
                }
            }
            off += 1;
        }

        //逆序求取第k个解码字符
        for &c in sb[..=off].iter().rev() {
            k %= decode_len;
            if k == 0 && c.is_ascii_alphabetic() {
                return char::from(c).to_string();
            }
            if c.is_ascii_digit() {
                let n = (c - b'0') as usize;
                decode_len /= n;
            } else {
                decode_len -= 1;
            }
        }

        return "".to_string();
    }
}
// @lc code=end
