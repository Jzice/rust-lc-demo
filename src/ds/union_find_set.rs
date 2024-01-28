//! # 并查集
//!

/// 并查集
pub struct UnionFindSet {
    // 元素个数
    n: usize,           
    // 集合数组
    pa: Vec<usize>,     
}

impl UnionFindSet {

    /// 新建大小为n的并查集
    pub fn new(n: usize) -> Self {
        UnionFindSet {
            n,
            pa: (0..n).collect::<Vec<_>>(),
        }
    }

    /// 并查集中不同集的个数
    pub fn size(&self) -> usize {
        self.n
    }

    /// 查找元素在哪个集中
    pub fn find(&mut self, a: usize) -> usize {
        let mut a_ = a;
        while self.pa[a_] != a_ {
            a_ = self.pa[a_];
        }
        self.pa[a] = a_;

        a_
    }

    /// 合并两个元素, 使之在同一个集合中
    pub fn union(&mut self, a: usize, b: usize) {
        let a_ = self.find(a); 
        let b_ = self.find(b);
        if a_ != b_ {
            self.pa[a_] = b_;
            self.n -= 1;
        }
    }

    /// 判断两个元素是否在同一个集中
    pub fn is_connect(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut ufs = UnionFindSet::new(10);

        assert!(ufs.size() == 10);
        assert!(ufs.find(0) == 0);
        assert!(ufs.find(5) == 5);
        assert!(ufs.size() == 10);
        ufs.union(0, 5);
        assert!(ufs.size() == 9);
        assert!(ufs.is_connect(0, 5));
        assert!(ufs.find(0) == 5);
        assert!(ufs.find(5) == 5);
    }
}

