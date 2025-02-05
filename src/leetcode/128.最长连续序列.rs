/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 *
 * https://leetcode.cn/problems/longest-consecutive-sequence/description/
 *
 * algorithms
 * Medium (54.45%)
 * Likes:    1011
 * Dislikes: 0
 * Total Accepted:    192.9K
 * Total Submissions: 354.3K
 * Testcase Example:  '[100,4,200,1,3,2]'
 *
 * 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
 *
 * 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [100,4,200,1,3,2]
 * 输出：4
 * 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,3,7,2,5,8,4,6,0,1]
 * 输出：9
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0
 * -10^9
 *
 *
 */

// @lc code=start
use std::collections::HashMap;

struct UnionFindSet {
    pa: HashMap<i32, i32>,
    n: usize,
}

impl UnionFindSet {
    pub fn new(nums: &Vec<i32>) -> Self {
        UnionFindSet {
            pa: nums.iter().fold(HashMap::new(), |mut pa, &n| {
                pa.insert(n, n);
                pa
            }),
            n: nums.len(),
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn find(&mut self, a: i32) -> Option<i32> {
        if !self.pa.contains_key(&a) {
            return None;
        }
        let mut a_ = a;
        while self.pa[&a_] != a_ {
            a_ = self.pa[&a_];
        }
        self.pa.insert(a, a_);

        Some(a_)
    }

    pub fn union(&mut self, a: i32, b: i32) {
        let (a_, b_) = (self.find(a), self.find(b));
        if a_ != b_ {
            self.pa.insert(a_.unwrap(), b_.unwrap());
            self.n -= 1;
        }
    }
}

impl Solution {
    /// - 并查集
    pub fn longest_consecutive1(nums: Vec<i32>) -> i32 {
        let mut ufs = UnionFindSet::new(&nums);
        // 合并连续元素
        for &n in &nums {
            if let Some(l) = ufs.find(n + 1) {
                ufs.union(n, n + 1);
            }
        }

        // 计算每个集中最大元素个数
        let mut res = 0;
        for &n in &nums {
            if let Some(l) = ufs.find(n) {
                res = res.max(l - n + 1);
            }
        }

        res
    }

    /// ## 解题思路
    /// - hashmap
    /// 1. 使用hashmap记录每个num对应的连续序列长度；
    /// 2. 从左至右顺序遍历数组；
    /// 3. 遍历过程中，使用hashmap记录当前数遍历时的最长连续序列长度；
    /// 4. 当前数的最长连续序列长度cur_long 为其左右数的最长连续序列长度left + right + 1;
    /// 5. 同时更新左右相邻区间的记录值(连起来了)；
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hash = HashMap::<i32, i32>::new();

        let mut longest = 0;
        for n in nums {
            if !hash.contains_key(&n) {
                let left = *hash.get(&(n - 1)).unwrap_or(&0);
                let right = *hash.get(&(n + 1)).unwrap_or(&0);

                let cur_long = left + right + 1;
                longest = longest.max(cur_long);
                hash.insert(n - left, cur_long);
                hash.insert(n, cur_long);
                hash.insert(n + right, cur_long);
            }
        }

        longest
    }
}
// @lc code=end

struct Solution;
mod tests {
    use super::*;
    fn test() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}
