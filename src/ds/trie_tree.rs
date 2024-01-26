// trie_tree.rs

use std::collections::HashMap;

/*
 *
 *          -----------------------
 *    root  | C1 | C2 |    | ... |  |
 *          ------------------------
 *            /
 *           /
 *  -------------------
 *  | B1 | B2 | ... |  |
 *
 * */

/// 前缀树节点
#[derive(Default)]
struct TrieNode {
    chars: HashMap<char, TrieNode>,
    is_word: bool,
}

/// 前缀树
struct TrieTree {
    root: TrieNode,
}

impl TrieTree {
    /// 新建trie tree
    pub fn new() -> Self {
        TrieTree {
            root: TrieNode::default(),
        }
    }

    /// 插入字符串
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.chars.entry(c).or_insert(TrieNode::default());
        }
        node.is_word = true;
    }

    /// 查找字符串
    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.chars.get(&c) {
                node = n;
            } else {
                return false;
            }
        }

        node.is_word
    }

    /// 是否存在前缀
    pub fn start_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(n) = node.chars.get(&c) {
                node = n;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut trie_tree1 = TrieTree::new();

        assert!(!trie_tree1.search("abc".into()));
        trie_tree1.insert("abc".into());
        trie_tree1.insert("abdf".into());
        assert!(trie_tree1.search("abc".into()));
        assert!(!trie_tree1.search("aef".into()));
        assert!(!trie_tree1.search("ab".into()));
        assert!(trie_tree1.start_with("ab".into()));
    }
}
