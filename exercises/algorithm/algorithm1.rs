/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd,
    {
        unsafe {
            let mut current_a = list_a.start;
            let mut current_b = list_b.start;
            let mut result = Self::new();
            result.length = list_a.length + list_b.length;

            // current_merged_ptr 始终指向已合并部分的最后一个节点。
            let mut current_merged_ptr: Option<NonNull<Node<T>>> = None;

            // --- 1. 确定合并链表的起始节点 ---
            let initial_choice = loop {
                match (current_a, current_b) {
                    (Some(a_ptr), Some(b_ptr)) => {
                        let a_val = &(*a_ptr.as_ptr()).val;
                        let b_val = &(*b_ptr.as_ptr()).val;

                        // 比较值并选择较小的节点
                        if a_val <= b_val {
                            current_a = (*a_ptr.as_ptr()).next; // 推进 A 的指针
                            break Some(a_ptr);
                        } else {
                            current_b = (*b_ptr.as_ptr()).next; // 推进 B 的指针
                            break Some(b_ptr);
                        }
                    }
                    // 如果其中一个链表为空，则起始节点是另一个链表的头
                    (Some(a_ptr), None) => break Some(a_ptr),
                    (None, Some(b_ptr)) => break Some(b_ptr),
                    (None, None) => break None, // 两个链表都为空
                }
            };

            if initial_choice.is_none() {
                return Self::new(); // 返回空链表
            }

            // 设置结果链表的 start 和初始 merged_ptr
            result.start = initial_choice;
            current_merged_ptr = initial_choice;

            // --- 2. 循环合并剩余的节点 ---
            while current_a.is_some() && current_b.is_some() {
                let next_node_to_link: NonNull<Node<T>>;
                let a_ptr = current_a.unwrap();
                let b_ptr = current_b.unwrap();

                let a_val = &(*a_ptr.as_ptr()).val;
                let b_val = &(*b_ptr.as_ptr()).val;

                if a_val <= b_val {
                    next_node_to_link = a_ptr;
                    current_a = (*a_ptr.as_ptr()).next; // 推进 A
                } else {
                    next_node_to_link = b_ptr;
                    current_b = (*b_ptr.as_ptr()).next; // 推进 B
                }

                // 将已合并节点的 next 指针指向新选中的节点
                (*current_merged_ptr.unwrap().as_ptr()).next = Some(next_node_to_link);

                // 推进 current_merged_ptr
                current_merged_ptr = Some(next_node_to_link);
            }

            // --- 3. 连接剩余部分 (其中一个链表已耗尽) ---
            let remainder = current_a.or(current_b);

            if let Some(end_ptr) = current_merged_ptr {
                // 将 merged 链表的末尾连接到剩余部分的起始
                (*end_ptr.as_ptr()).next = remainder;
            }

            // --- 4. 确定最终的 end 指针 ---
            result.end = if remainder.is_some() {
                // 如果有剩余部分，则 end 是原链表的 end
                if current_a.is_some() {
                    list_a.end
                } else {
                    // current_b 必须是 Some
                    list_b.end
                }
            } else {
                // 如果没有剩余，则 end 是最后连接的节点
                current_merged_ptr
            };

            result
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
