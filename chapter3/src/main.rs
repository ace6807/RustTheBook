mod data_types;
mod functions;
mod control_flow;
mod fizzbuzz;
mod challenges;

fn main() {
    // data_types::main();
    // functions::main();
    // control_flow::main();
    println!("{:.2}", challenges::celcius_to_fahrenheit(-20.3));
    println!("{:.2}", challenges::fahrenheit_to_celcius(-20.3));
    println!("{}", challenges::nth_fibbonacci(3))
}
