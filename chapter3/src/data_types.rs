#[allow(dead_code)]
pub fn main(){

    // Variables
    let x: f64 = 2.0;
    let y = 3.0;
    println!("x: {}, y: {}", x, y);

    // Mutable variables
    let mut a = 5;
    println!("a: {}", a);
    a = 10;
    println!("a: {}", a);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("MAX_POINTS: {}, THREE_HOURS_IN_SECONDS: {}", MAX_POINTS, THREE_HOURS_IN_SECONDS);

    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Arithmetic
    let sum = 5 + 10;
    println!("sum: {}", sum);

    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    let product = 4 * 30;
    println!("product: {}", product);

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("quotient: {}, truncated: {}, remainder: {}", quotient, truncated, remainder);

    // Boolean
    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    // Character
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);


    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (501, 43.4, 31);

    println!("tup.1: {} tup2.2: {}", tup.1, tup2.2);
    println!("tup: {:?}", tup); // {:?} tells println! to use the Debug trait which is defined by the std library


    // Destructuring
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let (x, y, z) = tup2;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Array
    let a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    println!("a[0]: {}", a[0]);

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("a: {:?}", a);

    // Out of bounds
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);



}