#[allow(dead_code)]
pub fn main(){
    let x: f64 = 2.0;
    let y: f32 = 3.0;
    println!("x: {}, y: {}", x, y);

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
}