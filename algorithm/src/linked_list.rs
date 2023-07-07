// this implementation has a drawback that you cant use it to create a empty list
// But I cant work out how to fix this issue.
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum ListNode {
    Node(Box<LinkedList>),
    Nil,
}

#[derive(Debug, Clone)]
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
            next: ListNode::Nil,
        }
    }

    pub fn prepend(self, t: i32) -> Self {
        Self {
            value: t,
            next: ListNode::Node(Box::new(self)),
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

    pub fn delete(&mut self, t: i32) {
        match &mut self.next {
            ListNode::Node(list) => {
                if list.value == t {
                    self.next = list.next.clone();
                } else {
                    list.delete(t);
                }
            }
            ListNode::Nil => {
                panic!("un-implemented for empty list");
            }
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 1;
        if let ListNode::Node(list) = &self.next {
            count += list.count();
        }
        count
    }

    pub fn update(&mut self, index: usize, t: i32) {
        if index < self.count() {
            if index == 0 {
                self.value = t;
            } else {
                if let ListNode::Node(list) = &mut self.next {
                    list.update(index - 1, t);
                }
            }
        } else {
            panic!("out of boundry");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list() {
        let l: LinkedList = LinkedList::new(1);
        println!("{l:?}");

        let mut l2 = l.prepend(1).prepend(2).prepend(3).prepend(2);
        println!("{l2}");
        println!("{l2}");

        l2.append(5);
        l2.append(6);
        l2.append(7);

        println!("{l2}");
        println!("{l2}");

        l2.delete(2);
        println!("{l2}");
        l2.delete(3);
        println!("{l2}");
        l2.delete(7);
        println!("{l2}");
        println!("{}", l2.count());

        l2.update(0, 10);
        println!("{l2}");
        l2.update(4, 10);
        println!("{l2}");
    }
}
