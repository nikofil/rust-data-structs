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

    pub fn push_back(&mut self, elem: T) {
        let mut new_node = Node::new(elem);
        let old_tail = self.tail.take();
        new_node.prev = old_tail;
        let new = Rc::new(RefCell::new(new_node));
        new.borrow_mut().prev.as_ref().map_or_else(|| {
            self.head = Some(Rc::clone(&new));
        }, |node| node.borrow_mut().next = Some(Rc::clone(&new)));
        self.tail = Some(Rc::clone(&new));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take();
        head.map(|head| {
            match &head.borrow().next {
                None => {
                    self.tail.take();
                },
                Some(snd) => {
                    self.head = Some(Rc::clone(&snd));
                    let mut snd = snd.borrow_mut();
                    snd.prev = None;
                },
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let tail = self.tail.take();
        tail.map(|tail| {
            match &tail.borrow().prev {
                None => {
                    self.head.take();
                },
                Some(snd) => {
                    self.tail = Some(Rc::clone(&snd));
                    let mut snd = snd.borrow_mut();
                    snd.next = None;
                },
            }
            Rc::try_unwrap(tail).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(String::from("first"));
        list.push_front(String::from("second"));
        let head = list.head.as_ref().unwrap().borrow();
        let tail = list.tail.as_ref().unwrap().borrow();
        assert_eq!(head.elem, "second");
        assert_eq!(tail.elem, "first");
    }

    #[test]
    fn test_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(String::from("first"));
        list.push_front(String::from("second"));
        list.push_back(String::from("third"));
        let head = list.head.as_ref().unwrap().borrow();
        let tail = list.tail.as_ref().unwrap().borrow();
        assert_eq!(head.elem, "second");
        assert_eq!(head.next.as_ref().unwrap().borrow().elem, "first");
        assert_eq!(tail.elem, "third");
    }

    #[test]
    fn test_pop() {
        let mut list = DoublyLinkedList::new();
        list.push_front(String::from("first"));
        list.push_front(String::from("second"));
        list.push_back(String::from("third"));
        let first = list.pop_front();
        assert_eq!(first.unwrap(), "second");
        let first = list.pop_back();
        assert_eq!(first.unwrap(), "third");
        let first = list.pop_back();
        assert_eq!(first.unwrap(), "first");
        let first = list.pop_front();
        assert!(first.is_none());
    }

    #[test]
    #[should_panic]
    fn test_pop_panic() {
        let mut list = DoublyLinkedList::new();
        list.push_front(String::from("first"));
        let first_ref = Rc::clone(list.head.as_ref().unwrap());
        let _mut_borrow = first_ref.borrow_mut();
        list.pop_front();
    }
}
