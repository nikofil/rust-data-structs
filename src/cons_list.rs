use std::mem;

pub enum Link<T> {
    More(Box<Node<T>>),
    None,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct ConsList<T>(Link<T>);

impl<T> ConsList<T> {
    pub fn new() -> ConsList<T> {
        ConsList(Link::None)
    }

    pub fn push(&mut self, val: T) {
        let old_head = mem::replace(&mut self.0, Link::None);
        let new = Node{elem: val, next: old_head};
        self.0 = Link::More(Box::new(new));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.0, Link::None){
            Link::More(node) => {
                let node = *node;
                self.0 = node.next;
                Some(node.elem)
            },
            Link::None => None,
        }
    }
}

impl<T> Drop for ConsList<T> {
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.0, Link::None);
        while let Link::More(mut next_node) = link {
            link = mem::replace(&mut next_node.next, Link::None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let new: ConsList<i32> = ConsList::new();
        if let Link::More(_) = new.0 {
            panic!("Should be none");
        }
    }

    #[test]
    fn push_front_int() {
        let mut new = ConsList::new();
        new.push(10);
        new.push(20);
        new.push(30);
        if let Link::More(ref first_node) = new.0 {
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
