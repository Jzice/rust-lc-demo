//! # 链表

type ElemT = i32;
type ListLink = Option<Box<ListNode>>;

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
    head: ListLink,
}

impl SingleLinkedList {
    pub fn new() -> Self {
        SingleLinkedList {
            head: None,
        }
    }

    /// 往链表尾部增加元素
    pub fn push(&mut self, v: i32) {
        let p = self.tail_mut();
        p.unwrap().next.replace(Box::new(ListNode::new(v)));
    }

    /// 获取末尾节点引用
    pub fn tail_ref(&self) -> Option<&Box<ListNode>> {
        let mut p = self.head.as_ref();
        while let Some(_p) = p {
            if _p.next.is_none() {
                p = Some(_p);
                break;
            }
            p = _p.next.as_ref();
        }
        p
    }

    /// 获取末尾节点可变引用
    pub fn tail_mut(&mut self) -> Option<&mut Box<ListNode>> {
        let mut p = self.head.as_mut();
        while let Some(_p) = p {
            if _p.next.is_none() {
                p = Some(_p);
                break;
            }
            p = _p.next.as_mut();
        }
        p
    }

}

impl From<Vec<i32>> for SingleLinkedList {
    fn from(value: Vec<i32>) -> Self {
        let mut sl = SingleLinkedList::new();
        if value.len() == 0 {
            return sl;
        }
        let mut t = sl.tail_mut();
        for v in value {
            if let Some(_t) = t {
                _t.next.replace(Box::new(ListNode::new(v)));
                t = _t.next.as_mut();
            }
        }
        sl
    }
}


impl Iterator for SingleLinkedList {
    type Item = ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sl1 = SingleLinkedList::from(vec![1, 2, 3]);

    }
}
