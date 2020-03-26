

pub fn strings() {
    println!("\n==============================================================");
    println!("Strings");
    println!("==============================================================");

    string_literals();
    byte_strings();
    strings_in_memory();
}

fn string_literals() {
    let string_literal = "this is a string";

    let string_literal_span_lines = "this is \
                                            a long string";

    // No escape characters required
    let string_literal_raw = r"C:\Program Files\Gorillas";

    // Multi-line starts and ends with hash symbols
    let raw_multi_line = r###"
                                This raw string started with 'r###"'.
                                Therefore it does not end until we reach a quote mark ('"')
                                followed immediately by three pound signs ('###'):
                            "###;
    println!("string_literal: {}", string_literal);
    println!("string_literal_span_lines: {}", string_literal_span_lines);
    println!("string_literal_raw: {}", string_literal_raw);
    println!("raw_multi_line: {}", raw_multi_line);


}

fn byte_strings() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}

// A &str (pronounced “stir” or “string slice”) is a reference to a run
// of UTF-8 text owned by someone else: it “borrows” the text.
//
// Like other slice references, a &str is a fat pointer, containing both the address
// of the actual data and its length

fn strings_in_memory() {
    let noodles = "noodles".to_string();

    // oodles is a &str referring to the last six bytes of the text belonging to noodles
    let oodles = &noodles[1..];

    let poodles = "ಠ_ಠ";

    println!("noodles: {}", noodles);
    println!("oodles: {}", oodles);
    println!("poodles: {}", poodles);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_string_literal() {
        let string_literal = "this is a string";
        assert_eq!(string_literal, "this is a string");
    }

    #[test]
    fn test_string_literal_span_lines() {
        let string_literal_span_lines =  "this is \
                                            a long string";
        assert_eq!(string_literal_span_lines, "this is a long string");
    }
}