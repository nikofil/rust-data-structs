type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None
        });
        let tail_raw = &mut *new_tail as *mut Node<T>;
        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        self.tail = tail_raw;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|h| {
            let h = *h;
            self.head = h.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            h.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut que = Queue::new();
        que.push(String::from("a"));
        que.push(String::from("b"));
        assert_eq!(que.pop().unwrap(), String::from("a"));
        assert_eq!(que.pop().unwrap(), String::from("b"));
        assert!(que.pop().is_none());
    }
}
