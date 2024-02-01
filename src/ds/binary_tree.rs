//! # 二叉树

use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

/// 二叉树节点
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// 新建二叉树节点
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

/// 数组转化为二叉树
pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    if vec.len() == 0 {
        return None;
    }
    if let Some(val) = vec[0] {
        let mut queue = VecDeque::new();
        let head = Rc::new(RefCell::new(TreeNode::new(val)));
        queue.push_back(Some(head.clone()));

        // 每次从数组中取两个, 作为队列头节点的左右子节点
        for children in vec[1..].chunks(2) {
            match queue.pop_front() {
                Some(p) if let Some(parent) = p.clone() => {
                    if let Some(v) = children[0] {
                        let left = Rc::new(RefCell::new(TreeNode::new(v)));
                        parent.borrow_mut().left = Some(left.clone()); 
                        queue.push_back(Some(left));
                    }
                    if children.len() > 1 {
                        if let Some(v) = children[1] {
                            let right = Rc::new(RefCell::new(TreeNode::new(v)));
                            parent.borrow_mut().right = Some(right.clone());
                            queue.push_back(Some(right));
                        }
                    }
                }
                Some(_) => {
                    queue.push_back(None);
                }
                None => {
                        }
            }
        }
        Some(head)
    } else {
        return None;
    }
}

#[macro_export]
macro_rules! btree {
    () => { None };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}

/// 前序遍历迭代器
pub struct PreorderIter<'a> {
    stack: Vec<&'a Rc<RefCell<TreeNode>>>,
}

impl<'a> PreorderIter<'a> {
    pub fn new(root: Option<&'a Rc<RefCell<TreeNode>>>) -> Self {
        root.map_or(Self { stack: vec![] }, |root| Self { stack: vec![root] })
    }
}

impl<'a> Iterator for PreorderIter<'a> {
    type Item = &'a Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // self.stack.pop().map(|node| {
        //     node.borrow()
        //         .right
        //         .as_ref()
        //         .map(|right| self.stack.push(right));
        //     node.borrow()
        //         .left
        //         .as_ref()
        //         .map(|left| self.stack.push(&left));
        //     node
        // })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //let mut t1 = tree!(vec![1, 2, 3]);
        //let t1_pre_iter = PreorderIter::new(t1.as_ref());

        // assert_eq!(t1_pre_iter.next(), Some(Rc::new(RefCell::new(TreeNode::new(1)))).as_ref());
    }
}

