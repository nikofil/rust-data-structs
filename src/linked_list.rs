use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;

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

    pub fn head(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|h| Ref::map(h.borrow(), |x| &x.val))
    }

    pub fn tail(&self) -> LinkedList<T> {
        let tail = self.head.as_ref().and_then(
            |first| first.borrow().next.as_ref().map(|x| Rc::clone(x))
        );
        LinkedList{head: tail}
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(node) = next {
            next = if let Ok(node) = Rc::try_unwrap(node) {
                node.borrow_mut().next.take()
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.insert(1);
        assert_eq!(&*list.head().unwrap(), &1);
        list.insert(2);
        assert_eq!(&*list.head().unwrap(), &2);
    }

    #[test]
    fn test_tail() {
        let mut list = LinkedList::new();
        list.insert(String::from("a"));
        list.insert(String::from("b"));
        let tail = list.tail();
        assert_eq!(&*tail.head().unwrap(), "a");
    }
}
