pub fn main() {

    #[allow(unused_mut)]
    let mut s = String::from("Foo bar zaz"); 

    // Naive approach using Strings
    let ret = first_word_1(&s);
    println!("{}", ret);

    let ret = second_word_1(&s);
    println!("{}", ret);

    // Better approach using slices
    let ret = first_word_slices(&s);
    println!("{}", ret);
    // s.clear(); // This would generate a compile time error!
    println!("{}", ret);

    let ret = second_word_slices(&s);
    println!("{}", ret);
    
}

fn first_word_slices(s: &str) -> &str {
    for (index, letter) in s.chars().enumerate() {
        if letter == ' ' {
            return &s[0..index];
        }
    }

    return &s[..];
}

fn second_word_slices(s: &str) -> &str {
    let mut start_index = 0;
    let mut ending_index = 0;

    for (index, letter) in s.chars().enumerate() {
        if letter == ' ' {
            if start_index == 0 {
                start_index = index;
            }
            else if ending_index == 0 {
                ending_index = index;
            }
        }
    }

    return &s[start_index+1..ending_index]
}


fn first_word_1(s: &String) -> String {
    if !s.contains(" ") {
        return s.to_string();
    }

    let mut first_word = String::default();
    for letter in s.chars() {
        if letter != ' '{
            first_word.push(letter);
        }
        else{
            return first_word;
        }
    }

    return first_word;
}

fn second_word_1(s: &String) -> String {
    if !s.contains(" ") {
        return String::default();
    }

    let mut second_word = String::default();
    let mut found_space = false;
    for letter in s.chars() {
        if letter == ' ' && !found_space {
            found_space = true;
        }        
        else if letter != ' ' && found_space {
            second_word.push(letter)
        }
        else if letter == ' ' && found_space {
            return second_word;
        } 
    }

    return second_word;
}