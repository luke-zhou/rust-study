/*
Remove Dups: Write code to remove duplicates from an unsorted linked list.
*/
use crate::linked_list::*;
use std::collections::HashSet;

pub fn run(list: &mut LinkedList)
{
    let mut h: HashSet<i32> = HashSet::new();
    h.insert(list.value);
    delete_dup(list, &mut h);
}

fn delete_dup(list: &mut LinkedList, dups: &mut HashSet<i32>){
    // println!();
    // println!("l: {list}");
    // println!("h: {dups:?}");

    if let ListNode::Node(l) = &mut list.next{
        // println!("v:{}", l.value);
        if dups.contains(&l.value){
            list.next = l.next.clone();
            delete_dup(list, dups);
        }else{
            dups.insert(l.value);
            delete_dup(l, dups);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dups() {
        let l: LinkedList = LinkedList::new(1);
        let mut l2 = l.prepend(1).prepend(2).prepend(3).prepend(2);
        println!("{l2}");
        run(&mut l2);
        println!("{l2}");
    }
}