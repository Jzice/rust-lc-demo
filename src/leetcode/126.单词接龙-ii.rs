/*!
 * # [126] 单词接龙 II
(
https://leetcode.cn/problems/word-ladder-ii/description/
)
 * 
 * @lc app=leetcode.cn id=126 lang=rust
 *
 * algorithms
 * Hard (37.75%)
 * Likes:    667
 * Dislikes: 0
 * Total Accepted:    55.2K
 * Total Submissions: 146.6K
 * Testcase Example:  '"hit"\n"cog"\n["hot","dot","dog","lot","log","cog"]'
 *
 * 按字典 wordList 完成从单词 beginWord 到单词 endWord 转化，一个表示此过程的 转换序列 是形式上像 beginWord ->
 * s1 -> s2 -> ... -> sk 这样的单词序列，并满足：
 *
 *
 *
 *
 * 每对相邻的单词之间仅有单个字母不同。
 * 转换过程中的每个单词 si（1 <= i <= k）必须是字典 wordList 中的单词。注意，beginWord 不必是字典 wordList
 * 中的单词。
 * sk == endWord
 *
 *
 * 给你两个单词 beginWord 和 endWord ，以及一个字典 wordList 。请你找出并返回所有从 beginWord 到 endWord
 * 的 最短转换序列 ，如果不存在这样的转换序列，返回一个空列表。每个序列都应该以单词列表 [beginWord, s1, s2, ..., sk]
 * 的形式返回。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log","cog"]
 * 输出：[["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
 * 解释：存在 2 种最短的转换序列：
 * "hit" -> "hot" -> "dot" -> "dog" -> "cog"
 * "hit" -> "hot" -> "lot" -> "log" -> "cog"
 *
 *
 * 示例 2：
 *
 *
 * 输入：beginWord = "hit", endWord = "cog", wordList =
 * ["hot","dot","dog","lot","log"]
 * 输出：[]
 * 解释：endWord "cog" 不在字典 wordList 中，所以不存在符合要求的转换序列。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= beginWord.length <= 5
 * endWord.length == beginWord.length
 * 1 <= wordList.length <= 500
 * wordList[i].length == beginWord.length
 * beginWord、endWord 和 wordList[i] 由小写英文字母组成
 * beginWord != endWord
 * wordList 中的所有单词 互不相同
 *
 *
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - bfs + dfs
    /// 1. 先通过bfs, 找到从begin_word到end_word的各级邻居节点列表;
    /// 2. 再通过dfs将分级邻居节点列表转换为最短路径列表;
    /// 3. bfs时, 双向同时
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::VecDeque;

        /// 检查w1, w2是否为邻居节点
        fn is_neighbour(w1: &str, w2: &str) -> bool {
            if w1.len() != w2.len() {
                return false;
            } else if w1.len() == 1 {
                return true;
            }
            w1.as_bytes()
                .iter()
                .zip(w2.as_bytes().iter())
                .filter(|(&b1, &b2)| b1 != b2)
                .count()
                == 1
        }

        // 如果begin_word, end_word为邻居
        if is_neighbour(&begin_word, &end_word) {
            return vec![vec![begin_word.clone(), end_word.clone()]];
        }

        // 如果单词表中没有end_word
        if word_list.iter().find(|&w| w == &end_word).is_none() {
            return vec![];
        }

        // 待检查节点
        let mut pending = word_list
            .into_iter()
            .filter(|x| x != &end_word && x != &begin_word)
            .collect::<VecDeque<String>>();

        // 从begin_word为起点逐级向外遍历时的某级节点
        let mut from = VecDeque::new();
        let mut from1 = VecDeque::new();
        from.push_back(begin_word.clone());
        // 从end_word开始,向外的遍历的某级节点
        let mut to = VecDeque::new();
        let mut to1 = VecDeque::new();
        to.push_back(end_word.clone());

        // 记录初始的from,to邻居节点
        let mut from_ladders = vec![]; // 出发->目标方向的接龙梯子
        let mut to_ladders = vec![]; // 目标->出发方向的接龙梯子
        from_ladders.push(from.iter().map(|s| s.clone()).collect::<Vec<String>>());
        to_ladders.insert(0, to.iter().map(|s| s.clone()).collect::<Vec<String>>());

        // 从两个方向同时对pending节点进行逐级分类
        let mut found = false;
        while !found && !pending.is_empty() {
            std::mem::swap(&mut from, &mut from1);
            std::mem::swap(&mut to, &mut to1);
            // 根据距离, 将所有未分类的word划分到from, to, pending三个集中
            for _ in 0..pending.len() {
                let pending_word = pending.pop_front().unwrap();
                let is_from_neigbour = from1.iter().any(|w1| is_neighbour(w1, &pending_word));
                let is_to_neigbour = to1.iter().any(|w1| is_neighbour(w1, &pending_word));
                if is_from_neigbour {
                    from.push_back(pending_word);
                    if is_to_neigbour {
                        found = true;
                    }
                } else if is_to_neigbour {
                    to.push_back(pending_word);
                } else {
                    pending.push_back(pending_word);
                }
            }
            // 如果pending中没有from,to两个方向的邻居, 则接龙中断
            if from.is_empty() && to.is_empty() {
                return vec![]; // 无法完成接龙, 返回空
            }
            // 如果pending中存在同时为from, to两个方向的邻居
            if found {
                to.clear(); // 则清理该层to方向的, 保留from方向的
            } else {
                // 否则不存在同时为连接两个方向的邻居节点
                // 检查刚收集的to, from两个中是否存在邻居关系
                for w in &to {
                    // 如果存在邻居关系
                    if from.iter().any(|w1| is_neighbour(w1, w)) {
                        found = true; //则找到最短路径
                        break;
                    }
                }
            }

            // 如果存在,记录from方向的邻居节点
            if !from.is_empty() {
                from_ladders.push(from.iter().map(|s| s.clone()).collect::<Vec<String>>());
            }
            // 如果存在,记录to方向的邻居节点
            if !to.is_empty() {
                to_ladders.insert(0, to.iter().map(|s| s.clone()).collect::<Vec<String>>());
            }
            // 清空旧的
            to1.clear();
            from1.clear();
        }

        // 检查完所有的pending节点后
        // 如果没有找到两个方向都相邻的节点
        if !found {
            // 则接龙无法完成
            return vec![]; //返回空
        }

        //否则找到接龙, 合并from, to两个方向遍历的邻居节点列表
        from_ladders.append(&mut to_ladders);

        // 将分级的邻居节点列表转换为路径列表
        fn dfs(ladders: &Vec<Vec<String>>, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            let level = path.len();
            if level == ladders.len() {
                res.push(path.clone());
                return;
            }

            for w in &ladders[level] {
                if is_neighbour(w, &path[level - 1]) {
                    path.push(w.clone());
                    dfs(ladders, path, res);
                    path.pop();
                }
            }
        }

        let mut res = vec![];
        dfs(&from_ladders, &mut vec![begin_word], &mut res);

        res
    }
}
// @lc code=end
