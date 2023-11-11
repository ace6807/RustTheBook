pub fn create_vectors(){
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);
    // v2.push(32); // Can't push to a non-mutable vector

    let mut v3 = vec![0; 10];
    println!("{:?}", v3);
    v3.push(32);
    println!("{:?}", v3);

    // create a vector using the macro and a range
    let v4: Vec<i32> = (1..10).collect();
    println!("{:?}", v4[3]);
}

pub fn access_vectors(){
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v[3]);
    println!("{:?}", v.get(3));
    // println!("{:?}", v[10]); // This will panic
    
    println!("{:?}", v.get(10)); // This will not panic. This gives us an Option
    match v.get(10) {
        Some(x) => println!("Item 10 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    let r = &v[1..3]; // This is a slice using a range
    println!("{:?}", r);
}

pub fn altering_vectors(){
    let mut v = vec![1, 2, 3, 4, 5];

    let mut a = &v[0];
    v.push(6);
    // println!("{:?}", a); // This will panic because we are borrowing a reference to v and then mutating v
}

pub fn iterating_over_vectors() {
    let v = vec![21, 333, 1121];
    print_vector(&v);

    let mut v3 = vec![1, 2, 3, 4, 5];
    increment_vector(&mut v3, 50);
    println!("{:?}", v3);
}

pub fn print_vector(v: &Vec<i32>) {
    println!("Printing vector");
    for i in v {
        println!("{}", i);
    }
    println!("Done printing vector");
}

pub fn increment_vector(v: &mut Vec<i32>, inc: i32) {
    for i in v {
        *i += inc;
    }
}

pub fn different_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}