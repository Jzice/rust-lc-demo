/*!
 * # [140.单词拆分II](https://leetcode.cn/problems/word-break-ii/description/)
 *
 * @lc app=leetcode.cn id=140 lang=rust
 *
 *
 * ## 难度
 *
 * - Hard (57.32%)
 * - Likes:    709
 * - Dislikes: 0
 * - Total Accepted:    91K
 * - Total Submissions: 158.6K
 * - Testcase Example:  '"catsanddog"\n["cat","cats","and","sand","dog"]'
 *
 * ## 问题描述
 *
 * 给定一个字符串 s 和一个字符串字典 wordDict ，在字符串 s 中增加空格来构建一个句子，使得句子中所有的单词都在词典中。以任意顺序
 * 返回所有这些可能的句子。
 *
 * 注意：词典中的同一个单词可能在分段中被重复使用多次。
 *
 *
 * ## 示例 1：
 *
 * - 输入:s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
 * - 输出:["cats and dog","cat sand dog"]
 *
 *
 * ## 示例 2：
 *
 *
 * - 输入:s = "pineapplepenapple", wordDict =
 * ["apple","pen","applepen","pine","pineapple"]
 * - 输出:["pine apple pen apple","pineapple pen apple","pine applepen apple"]
 * - 解释: 注意你可以重复使用字典中的单词。
 *
 *
 * ## 示例 3：
 *
 * - 输入:s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * - 输出:[]
 *
 *
 * ## 提示：
 *
 * - 1 <= s.length <= 20
 * - 1 <= wordDict.length <= 1000
 * - 1 <= wordDict[i].length <= 10
 * - s 和 wordDict[i] 仅有小写英文字母组成
 * - wordDict 中所有字符串都 不同
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 单词拆分
    /// ## 解题思路
    /// - 回溯法
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn dfs(s: &str, word_dict: &Vec<String>, tmp: &mut Vec<String>, res: &mut Vec<String>) {
            if s.is_empty() {
                res.push(tmp.clone().join(" "));
                return;
            }
            for w in word_dict {
                if let Some(0) = s.find(w) {
                    tmp.push(w.clone());
                    dfs(&s[w.len()..], word_dict, tmp, res);
                    tmp.pop();
                }
            }
        }

        let mut res = Vec::new();
        dfs(&s, &word_dict, &mut vec![], &mut res);

        res
    }
}
// @lc code=end
