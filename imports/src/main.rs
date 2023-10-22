mod math;
mod utils;

fn main() {
    math::hello();
    utils::hello();
    let res = math::math_utils::addition::add_nums(1, 2);
    println!("1 + 2 = {}", res);

    let name = utils::names::get_name();
    println!("Name: {}", name);

    let age = utils::names::add_two_to_age(30);
    println!("Age: {}", age);

    math::math_utils::addition::add_nums_and_print(1, 2);

    math::math_utils::subtraction::sub_nums_and_print(1, 2);
    let res = math::math_utils::subtraction::sub_nums(3, 3);
    println!("3 - 3 = {}", res);
}
