

// Structs are private by default, but can optionally add visibility
// specifiers to either the struct itself and/or its attributes

// NAMED FIELD structs. 
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}

pub struct QueueStd {
    pub older: Vec<char>,
    pub younger: Vec<char>,
}


// TUPLE-like structs
pub struct Bounds(pub usize, pub usize);

// UNIT structs
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
