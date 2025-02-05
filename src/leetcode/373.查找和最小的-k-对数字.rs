/*
 * @lc app=leetcode.cn id=373 lang=rust
 *
 * [373] 查找和最小的 K 对数字
 *
 * https://leetcode.cn/problems/find-k-pairs-with-smallest-sums/description/
 *
 * algorithms
 * Medium (40.66%)
 * Likes:    505
 * Dislikes: 0
 * Total Accepted:    61.4K
 * Total Submissions: 151K
 * Testcase Example:  '[1,7,11]\n[2,4,6]\n3'
 *
 * 给定两个以 非递减顺序排列 的整数数组 nums1 和 nums2 , 以及一个整数 k 。
 *
 * 定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2 。
 *
 * 请找到和最小的 k 个数对 (u1,v1),  (u2,v2)  ...  (uk,vk) 。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 * 输出: [1,2],[1,4],[1,6]
 * 解释: 返回序列中的前 3 对数：
 * ⁠    [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 *
 *
 * 示例 2:
 *
 *
 * 输入: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * 输出: [1,1],[1,1]
 * 解释: 返回序列中的前 2 对数：
 * [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 *
 *
 * 示例 3:
 *
 *
 * 输入: nums1 = [1,2], nums2 = [3], k = 3
 * 输出: [1,3],[2,3]
 * 解释: 也可能序列中所有的数对都被返回:[1,3],[2,3]
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= nums1.length, nums2.length <= 10^5
 * -10^9 <= nums1[i], nums2[i] <= 10^9
 * nums1 和 nums2 均为升序排列
 * 1 <= k <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 堆
    /// 1. 先将所有nums1[i]+nums2[0]和对应下标(i,0)入小顶堆;
    /// 2. 每次先弹出堆顶元素, 并push到结果数组中;
    /// 3. 再将nums[i]+nums2[j+1]及下标入堆;
    /// 4. 循环k次,堆顶弹出的即为第k小数字对;
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let (m, n) = (nums1.len(), nums2.len());
        let mut res = vec![];
        let mut heap = BinaryHeap::with_capacity(m);

        for i in 0..m {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0 as usize)));
        }

        for _ in 0..k {
            if let Some(Reverse((_sum, i, j))) = heap.pop() {
                res.push(vec![nums1[i], nums2[j]]);
                if j + 1 < n {
                    heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
                }
            }
        }

        res
    }
}
// @lc code=end
