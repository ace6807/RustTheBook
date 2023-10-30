#[allow(dead_code)]

pub fn main(){
    another_function();
    function_2(5);
    print_labeled_measurement(5, 'm');
    println!("The value of x is: {}", five());
    println!("The value of x is: {}", add_one(5));
}

fn another_function() {
    println!("Another function")
}

fn function_2(x: i32){
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}