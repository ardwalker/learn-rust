


pub fn loops() {



}

fn while_condition() {
    let mut x = 0;
    while x < 10 {
        x = x + 1;
    }
}

fn while_let() {
    while let pattern = expr {

    }
}

fn looping() {
    loop {

    }
}

fn for_pattern() {
    // for pattern in collection {}

    for i in 0..20 {

    }

    let strings: Vec<String> = vec!["one","two"];
    for s in strings {  // each string is moved into s here

    }                   // and dropped here

    for rs in &strings {  // better, loop over ref to collection

    }

}

fn breaks() {

    'search:  // optional labelled lifetime
    for i in 1..20 {
        if i == 5 {
            break 'search;
        }
    }
}

fn continues() {

}

