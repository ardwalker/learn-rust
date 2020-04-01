
// Generic struct
pub struct QueueGeneric<T> {
    older: Vec<T>,
    younger: Vec<T>
}

// With an implementation
impl<T> QueueGeneric<T> {
    pub fn new() -> QueueGeneric<T> {
        QueueGeneric { older: Vec::new(), younger: Vec::new() }
    }

    // Takes a shared reference because it doesn't need to modify itself
    pub fn _is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // if a method wants to take ownership of self, it can take self by value
    pub fn _split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

    pub fn push(&mut self, item: T) {
        self.younger.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop() // without semi means return statement
    }

}


#[cfg(test)]
mod tests {
    use super::QueueGeneric;

    #[test]
    fn generic() {
        let mut g  = QueueGeneric::new();
        g.push("hello");
        assert_eq!(g.pop(), Some("hello"));
    }

}