#[allow(dead_code)]
pub fn fizzbuzz(n: i32) -> Vec<String>{
    let mut solutions: Vec<String> = Vec::new();
    for i in 1..=n {
        let mut s: String = String::new();
        if i % 3 == 0{
            s.push_str("Fizz");
        }
        if i % 5 == 0{
            s.push_str("Buzz");
        }
        if s == "" {
            s = i.to_string();
        }
        solutions.push(s);
    }

    return solutions;
}
