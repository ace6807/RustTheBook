fn main() {
    // Copy (vaules on the stack)
    let x = 32;
    let y = x;
    let z = y;
    println!("{x} {y} {z}");

    let a = "foo";
    let b = a;
    println!("{a} {b}");

    let x = 23;
    copies_ownership(x);
    println!("{x}");


    // Move (vaules on the heap)
    let s: String = String::from("Testing");
    let r = s;
    println!("{}", r);
    // println!("{}", s); This is an error

    let s = String::from("Hello World");
    takes_ownership(s);
    // println!("{s}"); // This is an error.
    
    let mut s: String = String::from("Hello again");
    s = takes_and_gives_ownership(s);
    println!("{s}");


    // Clone
    let s: String = String::from("This will be copied");
    let r = s.clone();
    println!("{s} {r}");


    // Borrowing with a reference
    let s = String::from("Hello!");
    borrows_ownership(&s);
    println!("{s}");

    // Many immutable references are fine
    let s = String::from("FOO");
    let s2 = &s;
    let s3 = &s;
    println!("{} {} {}", s, s2, s3);

    // Mutable references (can only have 1!)
    let mut s: String = String::from("Bar");
    let s3: &mut String = &mut s;
    s3.push_str("yes");
    println!("{}", s2);

    let mut s = String::from("Testing");
    mutable_reference(&mut s);
    println!("{}", s);
    
}


fn takes_ownership(mut s: String) {
    s.push('1');
    println!("{s}");
}

fn mutable_reference(s: &mut String){
    s.push_str(" mutable references");
}

fn takes_and_gives_ownership(s: String) -> String {
    println!("{s}");
    return s;
}

fn copies_ownership(x: i32) {
    println!("{x}");
}

fn borrows_ownership(s: &String) {
    println!("{s}");
}