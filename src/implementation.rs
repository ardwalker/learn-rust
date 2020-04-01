

use super::structs::*;


impl QueueStd {
    pub fn new() -> QueueStd {
        QueueStd {older: Vec::new(), younger: Vec::new()}
    }

    // Takes a shared reference because it doesn't need to modify itself
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // if a method wants to take ownership of self, it can take self by value
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    pub fn push(&mut self, character:char) {
        self.younger.push(character);
    }

    pub fn pop(&mut self) -> Option<char> {
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
    use super::QueueStd;

    #[test]
    fn push() {
        let mut queue : QueueStd = QueueStd::new();
        queue.push('a');
        assert_eq!(queue.pop(), Some('a'));
    }

    #[test]
    fn is_empty() {
        let mut queue : QueueStd = QueueStd::new();
        assert_eq!(queue.is_empty(), true);
        queue.push('b');
        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn split() {
        let mut queue = QueueStd::new();
        queue.push('a');
        queue.push('b');

        let split_off = queue.split();
        assert_eq!(split_off.1, vec!['a','b']);
        assert_eq!(split_off.0, vec![]);

        // This will not compile, split moved the reference.
        // assert_eq!(queue.is_empty(), true);
    }

}