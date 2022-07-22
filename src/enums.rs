
// Enums can be constructed with several variants:
#[derive(Debug)]
pub enum FlashMessage {
    Success,                                    // A unit variant
    Warning{ category: i32, message: &'static str },  // A struct variant
    Error(&'static str)                               // A tuple variant
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_unit() {
        let a = FlashMessage::Success;
        assert!(matches!(a, FlashMessage::Success));
    }

    #[test]
    fn enum_tuple() {
        let a = FlashMessage::Error("An error");
        assert!(matches!(a, FlashMessage::Error(x) if x == "An error"));
    }

    #[test]
    fn enum_tuple_pattern_match() {
        let a = FlashMessage::Error("An error");
        let b = match a {FlashMessage::Error(m) => m, _ => "No match"};
        assert_eq!(b, "An error");
    }

    #[test]
    fn enum_tuple_pattern_if_let() {
        let a = FlashMessage::Error("An error msg");
        let b : &str = if let FlashMessage::Error(x) = a {x} else {"incorrect"};
        assert_eq!(b, "An error msg");
    }

    #[test]
    fn enum_struct() {
        let a = FlashMessage::Warning{category: 0, message: "Warn"};
        assert!(matches!(a, FlashMessage::Warning{category: _x, message: y} if y == "Warn"));
    }
    
    #[test]
    fn enum_tuple_generalized() {
        let x: Result<i32, &str> = Err("This is an error");
        let a = match x {Result::Err(m) => m, _ => "No match"};
        let b = if let Result::Err(m) = x {m} else {"No match"};

        assert_eq!(a, "This is an error");
        assert_eq!(b, "This is an error");
        assert!(matches!(x, Err(e) if e =="This is an error"));
    }
}
