// this implementation has a drawback that you cant use it to create a empty list
// But I cant work out how to fix this issue.
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ListNode {
    Node(Box<LinkedList>),
    Nil,
}

#[derive(Debug)]
pub struct LinkedList {
    pub value: i32,
    pub next: ListNode,
}

impl Display for LinkedList {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.value, self.next)
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Node(list_node) => write!(f, "({})", list_node),
            Self::Nil => write!(f, "({})", "Nil"),
        }
    }
}

impl LinkedList {
    pub fn new(v: i32) -> Self {
        Self {
            value: v,
            next: ListNode::Nil
        }
    }

    pub fn prepend(self, t: i32) -> Self {
        Self {
            value: t,
            next: ListNode::Node(Box::new(self))
        }
    }

    pub fn append(&mut self, t: i32) {
        match self.next {
            ListNode::Node(ref mut list) => list.append(t),
            ListNode::Nil => {
                let list = Self::new(t);
                self.next = ListNode::Node(Box::new(list))
            }
        }
    }
}
