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

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter{ que: self }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter{ cur: self.head.as_ref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{ cur: self.head.as_mut() }
    }
}

pub struct IntoIter<T> {
    que: Queue<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.que.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    cur: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.cur.take().map(|c| {
            let c = &**c;
            self.cur = c.next.as_ref();
            &c.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    cur: Option<&'a mut Box<Node<T>>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.cur.take().map(|c| {
            let c = &mut **c;
            self.cur = c.next.as_mut();
            &mut c.elem
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

    #[test]
    fn test_iters() {
        let mut que = Queue::new();
        let v = vec![String::from("a"), String::from("b"), String::from("c")];
        for i in &v {
            que.push(i.clone());
        }
        for i in que.iter_mut() {
            let is = i.clone();
            i.push_str(&is);
        }
        for (i, s) in que.iter().enumerate() {
            let mut tmp = String::new();
            tmp.push_str(&v[i]);
            tmp.push_str(&v[i]);
            assert_eq!(&tmp, s);
        }
        for i in que.into_iter() {
            assert_eq!(i.len(), 2);
        }
    }
}
