

#[cfg(test)]
mod tests {

    #[test]
    fn while_condition() {
        let mut x = 0;
        while x < 1 {
            x = x + 1;
        }
        assert_eq!(x, 1);
    }


    #[test]
    #[ignore]
    fn while_let() {
        unimplemented!();

        // while let pattern = expr {

        // }
    }

    #[test]
    #[ignore]
    fn looping() {
        unimplemented!();
        // loop {
        // }
    }

    #[test]
    #[ignore]
    fn for_pattern() {
        unimplemented!();
        // for pattern in collection {}

        // for i in 0..20 {

        // }

        // let strings: Vec<String> = vec!["one","two"];
        // for s in strings {  // each string is moved into s here

        // }                   // and dropped here

        // for rs in &strings {  // better, loop over ref to collection

        // }
    }

    #[test]
    #[ignore]
    fn breaks() {
        unimplemented!();

        // 'search:  // optional labelled lifetime
        // for i in 1..20 {
        //     if i == 5 {
        //         break 'search;
        //     }
        // }
    }

    #[test]
    #[ignore]
    fn continues() {
        unimplemented!();
    }
}


