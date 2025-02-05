/*!
 * # [211.添加与搜索单词-数据结构设计](https://leetcode.cn/problems/design-add-and-search-words-data-structure/description/)
 *
 * @lc app=leetcode.cn id=211 lang=rust
 *
 * ## 难度
 * - Medium (49.61%)
 * - Likes:    506
 * - Dislikes: 0
 * - Total Accepted:    73K
 * - Total Submissions: 147.2K
 * - Testcase Example:  '["WordDictionary","addWord","addWord","addWord","search","search","search","search"]\n' + '[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]'
 *
 * ## 问题描述
 *
 * 请你设计一个数据结构，支持 添加新单词 和 查找字符串是否与任何先前添加的字符串匹配 。
 * 
 * 实现词典类 WordDictionary ：
 * 
 * ```cpp
 * WordDictionary();   // 初始化词典对象
 * void addWord(word); // 将 word 添加到数据结构中，之后可以对它进行匹配
 * bool search(word);  // 如果数据结构中存在字符串与 word 匹配，则返回 true ；否则，返回  false 。word 中可能包含一些
 * '.' ，每个 . 都可以表示任何一个字母。
 * ```
 * 
 * ## 示例：
 * 
 * - 输入：
 * ```text
 * ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
 * [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
 * ```
 * - 输出：
 * [null,null,null,null,false,true,true,true]
 * 
 * - 解释：
 * ```cpp
 * WordDictionary wordDictionary = new WordDictionary();
 * wordDictionary.addWord("bad");
 * wordDictionary.addWord("dad");
 * wordDictionary.addWord("mad");
 * wordDictionary.search("pad"); // 返回 False
 * wordDictionary.search("bad"); // 返回 True
 * wordDictionary.search(".ad"); // 返回 True
 * wordDictionary.search("b.."); // 返回 True
 * ```
 * 
 * ## 提示：
 * - 1 <= word.length <= 25
 * - addWord 中的 word 由小写英文字母组成
 * - search 中的 word 由 '.' 或小写英文字母组成
 * - 最多调用 10^4 次 addWord 和 search
 * 
 * 
 */

// @lc code=start
use std::{collections::HashMap, default};

#[derive(Default)]
struct TrieNode {
    flag: bool,
    next: [OptNode; 26],
}

type OptNode = Option<Box<TrieNode>>;

struct WordDictionary {
    root: OptNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
impl WordDictionary {
    fn new() -> Self {
        Self { root: Some(Box::new(TrieNode::default())) }
    }
    
    fn add_word(&mut self, word: String) {
        let mut opt_node_ref = &mut self.root;
        for c in word.as_bytes() {
            let i = (*c - b'a') as usize;
            opt_node_ref = &mut opt_node_ref.as_mut().unwrap().next[i];
            if opt_node_ref.is_none() {
                *opt_node_ref = Some(Box::new(TrieNode::default()));
            }
        }
        opt_node_ref.as_mut().unwrap().flag = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut pending_nodes = vec![&self.root];
        let mut next_pending_nodes = Vec::new();
        for b in word.bytes() {
            if b == b'.' {
                // 如果当前字符为'.'
                // 则依次遍历待遍历的节点
                for node in pending_nodes.drain(..) {
                    //依次检查当前节点next中的所有节点,如果不为none, 则将
                    for next_node in node.as_ref()
                        .unwrap()
                        .next
                        .iter()
                        .filter(|o| o.is_some()) {
                            next_pending_nodes.push(next_node);
                    }
                }
            } else { // 当前为普通字符
                let i = (b - b'a') as usize;
                // 检查待遍历节点,
                for node in pending_nodes.drain(..) {
                    if node.as_ref().unwrap().next[i].is_some() {
                        next_pending_nodes.push(&node.as_ref().unwrap().next[i]);
                    }
                }
            }
            std::mem::swap(&mut pending_nodes, &mut next_pending_nodes);
        }

        pending_nodes.iter()
         .any(|n|n.as_ref().unwrap().flag)
    }
}

// @lc code=end

