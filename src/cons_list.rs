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
    fn pop_ints() {
        let mut new = ConsList::new();
        new.push(String::from("a"));
        new.push(String::from("b"));
        new.push(String::from("c"));
        assert_eq!(new.pop(), Some(String::from("c")));
        assert_eq!(new.pop(), Some(String::from("b")));
        assert_eq!(new.pop(), Some(String::from("a")));
        assert_eq!(new.pop(), None);
    }
}
