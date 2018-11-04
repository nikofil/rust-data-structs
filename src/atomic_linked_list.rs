use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

pub struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn val(&self) -> &T {
        &self.val
    }
}

pub struct AtomicLinkedList<T> {
    head: Link<T>,
}

impl<T> AtomicLinkedList<T> {
    pub fn new() -> AtomicLinkedList<T> {
        AtomicLinkedList{head: None}
    }

    pub fn insert(&mut self, val: T) {
        let old_head = self.head.take();
        let new_node = Node{val, next: old_head};
        self.head = Some(Arc::new(Mutex::new(new_node)));
    }

    pub fn head(&self) -> Option<MutexGuard<Node<T>>> {
        self.head.as_ref().map(|i| i.lock().unwrap())
    }

    pub fn tail(&self) -> AtomicLinkedList<T> {
        let tail = self.head.as_ref().and_then(
            |first| first.lock().unwrap().next.as_ref().map(|x| Arc::clone(x))
        );
        AtomicLinkedList{head: tail}
    }

    pub fn len(&self) -> usize {
        let mut sz =0;
        let mut cur = self.head.as_ref().map(Arc::clone);
        while let Some(next) = cur.take() {
            let x = next.lock().unwrap();
            cur = x.next.as_ref().map(Arc::clone);
            sz += 1;
        }
        sz
    }
}

impl<T> Drop for AtomicLinkedList<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(node) = next {
            next = if let Ok(node) = Arc::try_unwrap(node) {
                node.lock().unwrap().next.take()
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn share_test() {
        let list_mut = Arc::new(Mutex::new(AtomicLinkedList::new()));
        {
            let mut list = list_mut.lock().unwrap();
            list.insert(String::from("a"));
            list.insert(String::from("b"));
        }
        let list_mut_t = Arc::clone(&list_mut);
        thread::spawn(move || {
            let mut list = list_mut_t.lock().unwrap();
            list.insert(String::from("c"));
        }).join().unwrap();
        let list = list_mut.lock().unwrap();
        assert_eq!(list.head().unwrap().val(), "c");
    }

    #[test]
    fn race_test() {
        let mut joins = Vec::new();
        let list_mut = Arc::new(Mutex::new(AtomicLinkedList::new()));
        for _ in 0..100 {
            let list_mut_t = Arc::clone(&list_mut);
            joins.push(thread::spawn(move || {
                let mut list = list_mut_t.lock().unwrap();
                list.insert(String::from("a"));
                list.insert(String::from("b"));
                list.insert(String::from("c"));
                list.insert(String::from("d"));
            }));
        }
        joins.into_iter().for_each(|h| {h.join().unwrap();});
        let list = list_mut.lock().unwrap();
        assert_eq!(list.len(), 400);
    }
}
