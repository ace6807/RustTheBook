pub fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    return celcius * (9.0/5.0) + 32.0;
}

pub fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * (5.0 / 9.0);
}


pub fn nth_fibbonacci(n: i32) -> i32 {
    let mut fib1 = 0;
    let mut fib2 = 1;

    let mut counter = 1;
    let mut current = 1;

    while counter < n {
        current = fib1 + fib2;

        // swap them
        let temp = fib1 + fib2;
        fib1 = fib2;
        fib2 = temp;

        counter += 1;
    }

    return current;
}