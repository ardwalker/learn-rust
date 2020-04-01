

// ============================================================================
// String literals (&str): 
//      - value of a string is known at compile time and can remain unchanged.
//      - &str is an immutable ref to a string and cannot change
// 
// String objects (String): 
//      - string object is defined in the standard library
//      - mutable/growable collection allocated on the heap
//      - for string values provided at runtime
// ============================================================================

// Use String   to modify the string or need ownership of it


pub fn strings() {
}


#[cfg(test)]
mod tests {
    // use super::*;

 
    // &str -> String
    // .to_string()  simple conversion and show you're making a String
    // .to_owned() simple conversion and take ownership.
    // String::from() explicit conversion
    // String::push_str() if you need to append to a String
    // format!() if you have predefined text that you wanted formatted
    // .into() if you want a simple conversion and don't care about the
    #[test]
    fn strref_to_string_conversions() {
        let my_string1 = String::from("Hello World!");
        let my_string2 : String = "Hello World!".into();  
        let my_string3 = "Hello World!".to_owned();
        let my_string4 = format!("Hello {}!", "World");


        assert_eq!(my_string1, "Hello World!");
        assert_eq!(my_string2, "Hello World!");
        assert_eq!(my_string3, "Hello World!");
        assert_eq!(my_string4, "Hello World!");
    }

    #[test]
    // String -> &str

    // &String types turn into &str if the method call needs it
    //  let my_str: &str = &my_string; to explicitly specify a &str type
    fn string_to_strref() {
        let my_string1         = String::from("Hello World!"); // String
        let my_string2         = &my_string1;                   // &String type
        let my_string3 : &str  = &my_string1;                   // &str type
        assert_eq!(my_string1, "Hello World!");
        assert_eq!(my_string2, "Hello World!");
        assert_eq!(my_string3, "Hello World!");
    }

    #[test]
    fn string_literal() {
        let string_literal = "this is a string";
        assert_eq!(string_literal, "this is a string");
    }

    #[test]
    fn string_literal_span_lines() {
        let string_literal_span_lines =  "this is \
                                            a long string";
        assert_eq!(string_literal_span_lines, "this is a long string");
    }

    #[test]
    fn raw_string() {
        // No escape characters required
        let string_literal_raw = r"C:\Program Files\";
        assert_eq!(string_literal_raw, r"C:\Program Files\");
    }


    #[test]
    fn multiline_text() {
        // Multi-line starts and ends with any number of hash symbols
        let raw_multi_line = r###"This raw string started with 'r###"'.
Ends at quote followed by pound signs ('###'):
"###;
        assert_eq!(raw_multi_line, "This raw string started with 'r###\"'.\nEnds at quote followed by pound signs ('###'):\n");
    }

    #[test]
    fn byte_strings() {
        let method = b"GET";
        assert_eq!(method, &[b'G', b'E', b'T']);
    }

}