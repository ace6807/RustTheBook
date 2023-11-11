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