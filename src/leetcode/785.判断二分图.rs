/*
 * @lc app=leetcode.cn id=785 lang=rust
 *
 * [785] 判断二分图
 *
 * https://leetcode.cn/problems/is-graph-bipartite/description/
 *
 * algorithms
 * Medium (51.56%)
 * Likes:    327
 * Dislikes: 0
 * Total Accepted:    43.4K
 * Total Submissions: 84.2K
 * Testcase Example:  '[[1,2,3],[0,2],[0,1,3],[0,2]]'
 *
 * 存在一个 无向图 ，图中有 n 个节点。其中每个节点都有一个介于 0 到 n - 1 之间的唯一编号。给你一个二维数组 graph ，其中
 * graph[u] 是一个节点数组，由节点 u 的邻接节点组成。形式上，对于 graph[u] 中的每个 v ，都存在一条位于节点 u 和节点 v
 * 之间的无向边。该无向图同时具有以下属性：
 *
 * 不存在自环（graph[u] 不包含 u）。
 * 不存在平行边（graph[u] 不包含重复值）。
 * 如果 v 在 graph[u] 内，那么 u 也应该在 graph[v] 内（该图是无向图）
 * 这个图可能不是连通图，也就是说两个节点 u 和 v 之间可能不存在一条连通彼此的路径。
 *
 *
 * 二分图 定义：如果能将一个图的节点集合分割成两个独立的子集 A 和 B ，并使图中的每一条边的两个节点一个来自 A 集合，一个来自 B
 * 集合，就将这个图称为 二分图 。
 *
 * 如果图是二分图，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
 * 输出：false
 * 解释：不能将节点分割成两个独立的子集，以使每条边都连通一个子集中的一个节点与另一个子集中的一个节点。
 *
 * 示例 2：
 *
 *
 * 输入：graph = [[1,3],[0,2],[1,3],[0,2]]
 * 输出：true
 * 解释：可以将节点分成两组: {0, 2} 和 {1, 3} 。
 *
 *
 *
 * 提示：
 *
 *
 * graph.length == n
 * 1
 * 0
 * 0
 * graph[u] 不会包含 u
 * graph[u] 的所有值 互不相同
 * 如果 graph[u] 包含 v，那么 graph[v] 也会包含 u
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 以任意节点A为起点，将其染为RED；
    /// 2. 将以A为端点的边的另一点染为GREEN;
    /// 3. 依次遍历所有点；
    /// 4. 若遇到未染色点，则染为相反颜色；
    /// 5. 若遇到已染色点，且颜色与另一端点相同，则false；
    /// 6. 所有点都成功染色，则true；
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        //let mut colors = vec![0; graph.len() * 2];
        todo!()
    }

    fn color_dfs(node: usize, color: i32, graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>) -> bool {
        todo!()
    }
}
// @lc code=end
