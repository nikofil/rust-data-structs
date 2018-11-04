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
        let mut new: ConsList<i32> = ConsList::new();
        new.push(10);
        new.push(20);
        new.push(30);
        if let Link::More(first_node) = new.0 {
            assert_eq!(first_node.elem, 30);
        } else {
            panic!("No first node");
        }
    }
}
