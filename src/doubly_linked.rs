use std::rc::Rc;
use std::cell::RefCell;

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node {
            elem,
            next: None,
            prev: None,
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail:None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut new_node = Node::new(elem);
        let old_head = self.head.take();
        new_node.next = old_head;
        let new = Rc::new(RefCell::new(new_node));
        new.borrow_mut().next.as_ref().map_or_else(|| {
            self.tail = Some(Rc::clone(&new));
        }, |node| node.borrow_mut().prev = Some(Rc::clone(&new)));
        self.head = Some(Rc::clone(&new));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(String::from("second"));
        list.push_front(String::from("first"));
        let head = list.head.as_ref().unwrap().borrow();
        let tail = list.tail.as_ref().unwrap().borrow();
        assert_eq!(head.elem, "second");
        assert_eq!(tail.elem, "first");
    }
}
