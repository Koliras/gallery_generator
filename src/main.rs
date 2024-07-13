use std::fs;

fn main() {
    let mut first_link = String::new();
    let mut second_link = String::new();
    for line in fs::read_to_string("./input.txt").unwrap().lines() {
        if first_link != String::new() {
            second_link = line.to_string();
        }
        if first_link == String::new() {
            first_link = line.to_string();
        }

        if second_link == String::new() {
            continue;
        }

        println!("first link: ");
        println!("{}", first_link);
        println!("second link: ");
        println!("{}", second_link);
        if second_link != String::new() {
            first_link = String::new();
            second_link = String::new();
        }
    }
}
