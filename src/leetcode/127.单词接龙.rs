use std::f32::consts::E;

/*
 * @lc app=leetcode.cn id=127 lang=rust
 *
 * [127] 单词接龙
 *
 * https://leetcode.cn/problems/word-ladder/description/
 *
 * algorithms
 * Hard (48.24%)
 * Likes:    1271
 * Dislikes: 0
 * Total Accepted:    189.5K
 * Total Submissions: 392.8K
 * Testcase Example:  '"hit"\n"cog"\n["hot","dot","dog","lot","log","cog"]'
 *
 * 字典 wordList 中从单词 beginWord 和 endWord 的 转换序列 是一个按下述规格形成的序列 beginWord -> s1 ->
 * s2 -> ... -> sk：
 *
 *
 * 每一对相邻的单词只差一个字母。
 * 对于 1 <= i <= k 时，每个 si 都在 wordList 中。注意， beginWord 不需要在 wordList 中。
 * sk == endWord
 *
 *
 * 给你两个单词 beginWord 和 endWord 和一个字典 wordList ，返回 从 beginWord 到 endWord 的 最短转换序列
 * 中的 单词数目 。如果不存在这样的转换序列，返回 0 。
 *
 *
 * 示例 1：
 *
 *
 * 输入：beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log","cog"]
 * 输出：5
 * 解释：一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog", 返回它的长度 5。
 *
 *
 * 示例 2：
 *
 *
 * 输入：beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log"]
 * 输出：0
 * 解释：endWord "cog" 不在字典中，所以无法进行转换。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= beginWord.length <= 10
 * endWord.length == beginWord.length
 * 1 <= wordList.length <= 5000
 * wordList[i].length == beginWord.length
 * beginWord、endWord 和 wordList[i] 由小写英文字母组成
 * beginWord != endWord
 * wordList 中的所有字符串 互不相同
 *
 *
 */
struct Solution;
// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - bfs+queue
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::VecDeque;
        let mut words = word_list.into_iter().collect::<VecDeque<String>>();
        let mut laddrs = VecDeque::with_capacity(words.len()); //
        laddrs.push_back(begin_word);  
        let mut step = 0_i32;
        while !laddrs.is_empty() {
            step += 1;
            for _ in 0..laddrs.len() {
                let w = laddrs.pop_front().unwrap();
                for _ in 0..words.len() {
                    let next = words.pop_front().unwrap();
                    if Solution::is_ladder(&w, &next) {
                        if *next == end_word {
                            step += 1;
                            return step;
                        } else {
                            laddrs.push_back(next);
                        }
                    } else {
                        words.push_back(next);
                    }
                }
            }
        }

        return 0;
    }

    fn is_ladder(w1: &str, w2: &str) -> bool {
        if w1.len() != w2.len() {
            return false;
        }
        w1.as_bytes()
            .iter()
            .zip(w2.as_bytes().iter())
            .filter(|(&b1, &b2)| b1 != b2)
            .count()
            == 1
    }
}
// @lc code=end
