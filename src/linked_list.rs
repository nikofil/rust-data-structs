use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList{head: None}
    }

    pub fn insert(&mut self, val: T) {
        let old_head = self.head.take();
        let new_node = Node{val, next: old_head};
        self.head = Some(Rc::new(RefCell::new(new_node)));
    }

    pub fn tail(&self) -> LinkedList<T> {
        let tail = self.head.as_ref().and_then(
            |first| first.borrow().next.as_ref().map(|x| Rc::clone(x))
        );
        LinkedList{head: tail}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.insert(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().val, 1);
        list.insert(2);
        assert_eq!(list.head.as_ref().unwrap().borrow().val, 2);
    }

    #[test]
    fn test_tail() {
        let mut list = LinkedList::new();
        list.insert(String::from("a"));
        list.insert(String::from("b"));
        let tail = list.tail();
        assert_eq!(tail.head.as_ref().unwrap().borrow().val, "a");
    }
}
