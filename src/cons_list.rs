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
    pub fn new(val: T) -> ConsList<T> {
        ConsList(Link::More(Box::new(Node{elem: val, next: Link::None})))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let new = ConsList::new(10);
        if let ConsList(Link::More(node_box)) = new {
            assert_eq!(node_box.elem, 10);
        } else {
            panic!("Wrong type of ConsList enum");
        }
    }
}
