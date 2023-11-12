pub fn create_strings() -> String {
    let s: String = String::from("foo");
    println!("Created a string: {}", s);
    return "foo".to_string();
}

pub fn update_strings(s: &mut String) {
    s.push_str("bar");
    s.push('!');
    println!("Updated a string {}", s);
}

pub fn concatenate_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("Concatenated strings: {}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("Formatted strings: {}", s7);
}

pub fn access_strings() {
    let s = String::from("hello");
    // s[0]; // error: cannot index into a value of type `std::string::String`
    let h = &s[0..1];
    println!("Accessed a string: {}", h);
}

pub fn iterate_over_strings() {
    let s = String::from("hello");
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }

    // We can't get the grapheme clusters directly
    let s = String::from("नमस्ते");
    for c in s.chars() {
        println!("{}", c);
    }    
}