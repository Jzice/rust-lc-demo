/*!
 * # [87.扰乱字符串](https://leetcode.cn/problems/scramble-string/description/)
 *
 * @lc app=leetcode.cn id=87 lang=rust
 *
 * ## 难度
 * - Hard (47.35%)
 * - Likes:    526
 * - Dislikes: 0
 * - Total Accepted:    58.1K
 * - Total Submissions: 122.7K
 * - Testcase Example:  '"great"\n"rgeat"'
 *
 * ## 描述
 *
 * 使用下面描述的算法可以扰乱字符串 s 得到字符串 t ：
 *
 * - 如果字符串的长度为 1 ，算法停止
 * - 如果字符串的长度 > 1 ，执行下述步骤：
 *   - 在一个随机下标处将字符串分割成两个非空的子字符串。即，如果已知字符串 s ，则可以将其分成两个子字符串 x 和 y ，且满足 s = x + y。
 *   - 随机 决定是要「交换两个子字符串」还是要「保持这两个子字符串的顺序不变」。即，在执行这一步骤之后，s 可能是 s = x + y 或者 s = y + x 。
 *   - 在 x 和 y 这两个子字符串上继续从步骤 1 开始递归执行此算法。
 *
 * 给你两个 长度相等 的字符串 s1 和 s2，判断 s2 是否是 s1 的扰乱字符串。如果是，返回 true ；否则，返回 false 。
 *
 * ## 示例 1：
 * - 输入：s1 = "great", s2 = "rgeat"
 * - 输出：true
 * - 解释：s1 上可能发生的一种情形是：
 * ```text
 * "great" --> "gr/eat" // 在一个随机下标处分割得到两个子字符串
 * "gr/eat" --> "gr/eat" // 随机决定：「保持这两个子字符串的顺序不变」
 * "gr/eat" --> "g/r / e/at" // 在子字符串上递归执行此算法。两个子字符串分别在随机下标处进行一轮分割
 * "g/r / e/at" --> "r/g / e/at" // 随机决定：第一组「交换两个子字符串」，第二组「保持这两个子字符串的顺序不变」
 * "r/g / e/at" --> "r/g / e/ a/t" // 继续递归执行此算法，将 "at" 分割得到 "a/t"
 * "r/g / e/ a/t" --> "r/g / e/ a/t" // 随机决定：「保持这两个子字符串的顺序不变」
 * ```
 * 算法终止，结果字符串和 s2 相同，都是 "rgeat"
 * 这是一种能够扰乱 s1 得到 s2 的情形，可以认为 s2 是 s1 的扰乱字符串，返回 true
 *
 *
 * ## 示例 2：
 * - 输入：s1 = "abcde", s2 = "caebd"
 * - 输出：false
 *
 * ## 示例 3：
 * - 输入：s1 = "a", s2 = "a"
 * - 输出：true
 *
 * ## 提示：
 * - s1.length == s2.length
 * - s1 和 s2 由小写英文字母组成
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 递归+merge
    /// 1. 设 f(s1, s2) 表示s1, s2是否为扰乱字符串;
    /// 2. 则 如果存在 划分 i,
    ///         将s1 划分为 s1[..i], s1[i..]
    ///         将s2 划分为 s2[..i], s2[i..]
    ///    如果 f(s1[..i], s2[..i]) && f(s1[i..], s2[i..]) => f(s1, s2) is true
    ///    或者划分后交换,
    ///        f(s1[..i], s2[-i..]) && f(s1[-i..], s2[..i])
    ///        
    pub fn is_scramble0(s1: String, s2: String) -> bool {
        use std::collections::HashMap;
        fn is_scramble_<'a>(
            s1: &'a str,
            s2: &'a str,
            cache: &mut HashMap<(&'a str, &'a str), bool>,
        ) -> bool {
            if let Some(&b) = cache.get(&(s1, s2)) {
                return b;
            }
            cache.insert((s1, s2), false);
            // 长度不相等, false
            if s1.len() != s2.len() {
                return false;
            }

            // 两个字符串相等时, 为true
            if s1 == s2 {
                cache.insert((s1, s2), true);
                return true;
            }
            // 一个字符串中的任何一个字符不在另一个字符串中, 为false
            if s1
                .as_bytes()
                .iter()
                .any(|&s1b| s2.as_bytes().iter().all(|&s2b| s1b != s2b))
                || s2
                    .as_bytes()
                    .iter()
                    .any(|&s1b| s1.as_bytes().iter().all(|&s2b| s1b != s2b))
            {
                return false;
            }

            let n = s1.len();
            for i in 1..n {
                // 存在一个划分i不交换, 使s1, s2经过i划分后对应的子字符串 为扰乱字符串
                if is_scramble_(&s1[..i], &s2[..i], cache)
                    && is_scramble_(&s1[i..], &s2[i..], cache)
                {
                    cache.insert((s1, s2), true);
                    return true;
                }
                // 存在一个划分i并交换, 使s1, s2经过i划分后对应的子字符串 为扰乱字符串
                if is_scramble_(&s1[..i], &s2[(n - i)..], cache)
                    && is_scramble_(&s1[i..], &s2[..(n - i)], cache)
                {
                    cache.insert((s1, s2), true);
                    return true;
                }
            }

            return false;
        }

        let mut cache = HashMap::new();
        is_scramble_(&s1, &s2, &mut cache)
    }

    /// ## 解题思路2
    /// - 动态规划
    /// 1. 设 dp[i][j][l]: 表示s1[i..(i+l)], s[j..(j+l)]是否为合法扰乱字符串
    /// 2. 递推关系:
    ///    dp[i][j][l] = dp[i][j][k]&&dp[i+k][j+k][l-k]
    ///                or dp[i][j+l-k][k]&&dp[i+l-k][j][l-k]
    ///                 ( 0<k<l)
    /// 3. 初始条件: dp[i][j][1] = s[i] == s[j] (0<=i, j < n),
    /// 4. 目标值: dp[0][0][n]
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut dp = vec![vec![vec![false; n + 1]; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                dp[i][j][1] = &s1[i..i + 1] == &s2[j..j + 1];
            }
        }
        for l in 2..=n {
            for i in 0..=n - l {
                for j in 0..=n - l {
                    for k in 1..l {
                        if (dp[i][j][k] && dp[i + k][j + k][l - k])
                            || (dp[i][j + l - k][k] && dp[i + k][j][l - k])
                        {
                            dp[i][j][l] = true;
                            break;
                        }
                    }
                }
            }
        }

        dp[0][0][n]
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_scramble("great".to_string(), "rgeat".to_string()), true);
        
    }
}
