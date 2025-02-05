/*!
 * # [30.串联所有单词的子串](https://leetcode.cn/problems/substring-with-concatenation-of-all-words/description/)
 *
 * @lc app=leetcode.cn id=30 lang=rust
 *
 * ## 难度
 *
 * - Hard (36.16%)
 * - Likes:    576
 * - Dislikes: 0
 * - Total Accepted:    87.2K
 * - Total Submissions: 241.1K
 * - Testcase Example:  '"barfoothefoobarman"\n["foo","bar"]'
 *
 * ## 问题描述
 *
 * 给定一个字符串 s 和一些 长度相同 的单词 words 。找出 s 中恰好可以由 words 中所有单词串联形成的子串的起始位置。
 *
 * 注意子串要与 words 中的单词完全匹配，中间不能有其他字符 ，但不需要考虑 words 中单词串联的顺序。
 *
 *
 * ## 示例 1：
 *
 * - 输入：s = "barfoothefoobarman", words = ["foo","bar"]
 * - 输出：[0,9]
 * - 解释：
 * 从索引 0 和 9 开始的子串分别是 "barfoo" 和 "foobar" 。
 * 输出的顺序不重要, [9,0] 也是有效答案。
 *
 *
 * ## 示例 2：
 *
 * - 输入：s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
 * - 输出：[]
 *
 *
 * ## 示例 3：
 *
 * - 输入：s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
 * - 输出：[6,9,12]
 *
 *
 * # 提示：
 *
 * - s 由小写英文字母组成
 * - words[i] 由小写英文字母组成
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 串联所有单词的子串
    /// ## 解题思路
    /// - 滑动窗口+hashmap
    /// 1. hashmap记录遍历时,各个word出现的次数;
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
        let s = s.as_bytes();
        let str_limit = s.len();
        let word_cnt = words.len();
        let word_size = words[0].len();
        let mut res = Vec::new();
        //统计words中各个word的次数, word -> (word_cnt, checked)
        let mut word_map =
            words
                .iter()
                .fold(HashMap::<&[u8], (usize, usize)>::new(), |mut map, word| {
                    map.entry(word.as_bytes()).or_default().0 += 1;
                    map
                });

        // 检查各个窗口中的字符串是否符合
        for step in 0..word_size {
            let mut l = step;
            let mut cnt = 0; //
            // 重置word_map
            word_map.iter_mut().for_each(|(_, e)| {
                e.1 = 0;
            });
            // 检查窗口内
            while l + word_cnt * word_size <= str_limit {
                // 检查窗口内各个word是否
                while cnt < word_cnt {
                    match word_map.get_mut(&s[l+word_size*cnt..l + word_size*(cnt+1)].as_ref()) {
                        None => {   //当前字符串不存在
                            l += (cnt+1) * word_size;
                            cnt = 0;
                            word_map.iter_mut().for_each(|(_, e)| {
                                e.1 = 0;
                            });
                            break;
                        }
                        Some(entry) => { //
                            entry.1 += 1; //checked 计数-1
                            cnt += 1;
                            if entry.1 > entry.0 {
                                if let Some(e) = word_map.get_mut(&s[l..l+word_size]){
                                    e.1 -= 1;
                                }
                                l += word_size;   //滑动窗口左边界
                                cnt -= 1; //窗口内word计数-1
                                break;
                            }
                        }
                    }
                }

                // 如果检查了word_cnt个word 
                if cnt == word_cnt {
                    if word_map.values().find(|&e| e.1 != e.0).is_none() {
                        res.push(l as i32);
                    }
                    // 将窗口内最左边界word的当前窗口计数-1
                    if let Some(e) = word_map.get_mut(&s[l..l+word_size]){
                        e.1 -= 1;
                    }
                    l += word_size;   //滑动窗口左边界
                    cnt -= 1; //窗口内word计数-1
                }
            }
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_substring("wordgoodgoodgoodbestword".to_string(), 
                                            vec!["word".to_string(), 
                                            "good".to_string(),
                                            "best".to_string(),
                                            "good".to_string()]), 
                   vec![8]
                  );
    }
}
