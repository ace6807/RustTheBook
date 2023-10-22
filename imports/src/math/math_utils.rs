pub mod addition {
    pub fn add_nums(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    pub fn add_nums_and_print(num1: i32, num2: i32) {
        println!("{} + {} = {}", num1, num2, num1 + num2);
    }
}

pub mod subtraction {
    pub fn sub_nums(num1: i32, num2: i32) -> i32 {
        num1 - num2
    }

    pub fn sub_nums_and_print(num1: i32, num2: i32) {
        println!("{} - {} = {}", num1, num2, num1 - num2);
    }
}