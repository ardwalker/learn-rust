

pub struct QueueStd {
    pub older: Vec<char>,
    pub younger: Vec<char>,
}

// impl QueueStd {
//     pub fn new() -> QueueStd {
//         QueueStd {older: Vec::new(), younger: Vec::new()}
//     }
//
//     // Takes a shared reference because it doesn't need to modify itself
//     pub fn is_empty(&self) -> bool {
//         self.older.is_empty() && self.younger.is_empty()
//     }
//
//     // if a method wants to take ownership of self, it can take self by value
//     pub fn split(self) -> (Vec<char>, Vec<char>) {
//         (self.older, self.younger)
//     }
//
//     pub fn push(&mut self, character:char) {
//         self.younger.push(character);
//     }
//
//     pub fn pop(&mut self) -> Option<char> {
//         if self.older.is_empty() {
//             if self.younger.is_empty() {
//                 return None;
//             }
//
//             use std::mem::swap;
//             swap(&mut self.older, &mut self.younger);
//             self.older.reverse();
//         }
//         self.older.pop() // without semi means return statement
//     }
// }


// named field structs. private by default, but can optionally add visibility
// specifiers to either the struct itself and/or its attributes
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}

// tuple-like structs
pub struct Bounds(pub usize, pub usize);

// unit structs
struct _Onesuch;


#[cfg(test)]
mod tests {
    use super::*;

    fn _new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
        assert_eq!(pixels.len(), size.0 * size.1);
        GrayscaleMap { pixels, size }
    }

    #[test]
    fn named_field_struct() {
        let width = 1024;
        let height = 576;
        let image = GrayscaleMap {
            pixels: vec![0; width * height],
            size: (width, height)
        };

        assert_eq!(image.size,(1024,576));
    }

    #[test]
    fn tuple_like_structs() {
        let image_bounds = Bounds(1024, 768);
        assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    }

}
