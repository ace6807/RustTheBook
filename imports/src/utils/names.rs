use crate::math::math_utils::addition::add_nums;

pub fn get_name() -> String {
    return String::from("John");
}

pub fn add_two_to_age(age: i32) -> i32 {
    return add_nums(age, 2);
}