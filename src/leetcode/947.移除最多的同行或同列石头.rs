/*
 * @lc app=leetcode.cn id=947 lang=rust
 *
 * [947] 移除最多的同行或同列石头
 *
 * https://leetcode.cn/problems/most-stones-removed-with-same-row-or-column/description/
 *
 * algorithms
 * Medium (61.50%)
 * Likes:    324
 * Dislikes: 0
 * Total Accepted:    34.4K
 * Total Submissions: 56K
 * Testcase Example:  '[[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]'
 *
 * n 块石头放置在二维平面中的一些整数坐标点上。每个坐标点上最多只能有一块石头。
 *
 * 如果一块石头的 同行或者同列 上有其他石头存在，那么就可以移除这块石头。
 *
 * 给你一个长度为 n 的数组 stones ，其中 stones[i] = [xi, yi] 表示第 i 块石头的位置，返回 可以移除的石子
 * 的最大数量。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
 * 输出：5
 * 解释：一种移除 5 块石头的方法如下所示：
 * 1. 移除石头 [2,2] ，因为它和 [2,1] 同行。
 * 2. 移除石头 [2,1] ，因为它和 [0,1] 同列。
 * 3. 移除石头 [1,2] ，因为它和 [1,0] 同行。
 * 4. 移除石头 [1,0] ，因为它和 [0,0] 同列。
 * 5. 移除石头 [0,1] ，因为它和 [0,0] 同行。
 * 石头 [0,0] 不能移除，因为它没有与另一块石头同行/列。
 *
 * 示例 2：
 *
 *
 * 输入：stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
 * 输出：3
 * 解释：一种移除 3 块石头的方法如下所示：
 * 1. 移除石头 [2,2] ，因为它和 [2,0] 同行。
 * 2. 移除石头 [2,0] ，因为它和 [0,0] 同列。
 * 3. 移除石头 [0,2] ，因为它和 [0,0] 同行。
 * 石头 [0,0] 和 [1,1] 不能移除，因为它们没有与另一块石头同行/列。
 *
 * 示例 3：
 *
 *
 * 输入：stones = [[0,0]]
 * 输出：0
 * 解释：[0,0] 是平面上唯一一块石头，所以不可以移除它。
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 0 i, yi
 * 不会有两块石头放在同一个坐标点上
 *
 *
 */

struct Solution;

// @lc code=start
/// 并查集
struct UnionFindSet {
    n: usize,
    pa: Vec<usize>,
}

impl UnionFindSet {
    pub fn new(n: usize) -> Self {
        UnionFindSet {
            n,
            pa: (0..n).collect::<Vec<_>>(),
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn find(&mut self, a: usize) -> usize {
        let mut a_ = a;
        while self.pa[a_] != a_ {
            a_ = self.pa[a_];
        }
        self.pa[a] = a_;

        a_
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a_ = self.find(a);
        let b_ = self.find(b);
        if a_ != b_ {
            self.pa[a_] = b_;
            self.n -= 1;
        }
    }
}

impl Solution {
    /// ## 解题思路
    /// - 并查集+hashmap
    /// 1. 所有同行或同列的石头(节点)之间相当于存在一条边来表示之间存在关联;
    ///    所有有关联的边将节点聚集成为一个连通图, 最后剩下的石头数等于连通图个数;
    /// 2. 用并查集来记录各个节点是否在一个连通图中;
    /// 3. 用两个hashmap, 分别记录出现的节点的行列下标是否出现;
    /// 4. 顺序遍历所有节点, 如果行或列下标出现过, 则将节点id合并到并查集中;
    ///    否则, 将行列坐标记录到hashmap中;
    /// 5. 最后结果: 节点数n - 并查集大小
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut ufs = UnionFindSet::new(n);

        let mut row = std::collections::HashMap::new();
        let mut col = std::collections::HashMap::new();

        // 依次遍历各个石头
        for i in 0..n {
            let (r, c) = (stones[i][0], stones[i][1]);
            // 如果当前石子的行坐标有相同的, 则将下标其合并进并查集中
            if let Some(j) = row.get(&r) {
                ufs.union(*j, i);
            } else {
                // 否则没有相同的行, 则将(行号r, 节点下标i)记录到hashmap中
                row.insert(r, i);
            }

            if let Some(j) = col.get(&c) {
                ufs.union(*j, i);
            } else {
                col.insert(c, i);
            }
        }

        (n - ufs.size()) as i32
    }
}

// @lc code=end
