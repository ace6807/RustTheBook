#![allow(unused)]

mod vectors;
mod strings;

fn main() {
    // vectors::create_vectors();
    // vectors::access_vectors();
    // vectors::altering_vectors();
    // vectors::iterating_over_vectors();
    // vectors::different_types();
    let mut s = strings::create_strings();
    strings::update_strings(&mut s);
    strings::concatenate_strings();
    strings::access_strings();
    strings::iterate_over_strings();
}
