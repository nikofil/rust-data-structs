type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct ConsList<T>(Link<T>);

impl<T> ConsList<T> {
    pub fn new() -> ConsList<T> {
        ConsList(None)
    }

    pub fn push(&mut self, val: T) {
        let new = Node{elem: val, next: self.0.take()};
        self.0 = Some(Box::new(new));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.take().map(|node| {
            let node = *node;
            self.0 = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|head| &head.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|head| &mut head.elem)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter{cur: self.0.as_ref()}
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter{list: self}
    }
}

pub struct Iter<'a, T: 'a> {
    cur: Option<&'a Box<Node<T>>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.cur.take().map(|cur| {
            self.cur = cur.next.as_ref();
            &cur.elem
        })
    }
}

pub struct IntoIter<T> {
    list: ConsList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.list.pop()
    }
}

impl<T> Drop for ConsList<T> {
    fn drop(&mut self) {
        let mut link = self.0.take();
        while let Some(mut next_node) = link {
            link = next_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let new: ConsList<i32> = ConsList::new();
        if new.0.is_some() {
            panic!("Should be none");
        }
    }

    #[test]
    fn push_front_int() {
        let mut new = ConsList::new();
        new.push(10);
        new.push(20);
        new.push(30);
        if let Some(ref first_node) = new.0 {
            assert_eq!(first_node.elem, 30);
        } else {
            panic!("No first node");
        }
    }

    #[test]
    fn pop_strs() {
        let mut new = ConsList::new();
        new.push(String::from("a"));
        new.push(String::from("b"));
        new.push(String::from("c"));
        assert_eq!(new.pop(), Some(String::from("c")));
        assert_eq!(new.pop(), Some(String::from("b")));
        assert_eq!(new.pop(), Some(String::from("a")));
        assert_eq!(new.pop(), None);
    }

    #[test]
    fn peek_str() {
        let mut new = ConsList::new();
        assert_eq!(new.peek(), None);
        new.push(String::from("a"));
        assert_eq!(new.peek(), Some(&String::from("a")));
        assert_eq!(new.peek(), Some(&String::from("a")));
    }

    #[test]
    fn peek_mut_str() {
        let mut new = ConsList::new();
        new.push(String::from("a"));
        new.peek_mut().map(|s| s.push_str("bc"));
        assert_eq!(new.peek(), Some(&String::from("abc")));
    }

    #[test]
    fn into_iter() {
        let mut new = ConsList::new();
        new.push(10);
        new.push(20);
        new.push(30);
        let mut iter = new.into_iter();
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut new = ConsList::new();
        new.push(10);
        new.push(20);
        let v: Vec<&i32> = new.iter().collect();
        assert_eq!(v, vec![&20, &10]);
        let v: Vec<&i32> = new.iter().collect();
        assert_eq!(v, vec![&20, &10]);
    }
}
