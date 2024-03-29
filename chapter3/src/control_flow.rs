#[allow(dead_code)]
pub fn main(){
    let number = 12;

    if number < 5 {
        println!("The number is bigger than 5")
    }
    else {
        println!("The number is less than or equal to 5")
    }


    if number != 0 {
        println!("The number is something other than zero!");
    }
    else{
        println!("The number is zero!");
    }


    if number % 4 == 0 {
        println!("The number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("The number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("The number is divisible by 2");
    }
    else {
        println!("The number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number: i32 = if condition {5} else {6};

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the loop is: {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value at index {} is: {}", index, a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for element in a.iter().rev() {
        println!("The value is: {}", element);
    }

    for element in (1..4).rev() {
        println!("The value is: {}", element);
    }

}