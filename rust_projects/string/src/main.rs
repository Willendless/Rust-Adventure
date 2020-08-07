fn main() {
    let a = String::from("hello world");
    let b = a;
    println!("what is b ?: {}", b);


    let mut s1 = gives_ownership();
    println!("len of s1: {}", caculate_length(&s1));
    
    append_ok(&mut s1);

    let mut k1 = String::from("hello");
    let p1 = &k1;
    let p2 = &k1;
    println!("{} and {}", p1, p2);
    println!("{} and {}", p1, p2);
    // p1 and p2 are no longer used after this point

    let p3 = &mut k1; 
    println!("{}", p3);
    println!("first word: {}", first_word(p3));
    println!("first word2: {}", first_word2(p3));
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn caculate_length(s: &String) -> usize {
    s.len()
}

fn append_ok(s: &mut String) {
    s.push_str(" ok");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
