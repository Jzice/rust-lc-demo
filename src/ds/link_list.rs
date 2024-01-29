//! # 链表

type Link = Option<Box<ListNode>>;

/// 链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/// 单向链表
pub struct SingleLinkedList {
    /// 链表头
    head: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
