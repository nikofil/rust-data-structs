pub enum ConsList<T> {
    Elem(T, Box<ConsList<T>>),
    Nil,
}

impl<T> ConsList<T> {
    pub fn new(val: T) -> ConsList<T> {
        ConsList::Elem(val, Box::new(ConsList::Nil))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let new = ConsList::new(10);
        if let ConsList::Elem(val, _) = new {
            assert_eq!(val, 10);
        } else {
            panic!("Wrong type of ConsList enum");
        }
    }
}
