use std::fs;

fn main() {
    let mut source_link = String::new();
    let mut dest_link = String::new();
    for line in fs::read_to_string("./input.txt").unwrap().lines() {
        if source_link != String::new() {
            dest_link = line.to_string();
        }
        if source_link == String::new() {
            source_link = line.to_string();
        }

        if dest_link == String::new() {
            continue;
        }

        println!("first link: ");
        println!("{}", source_link);
        println!("second link: ");
        println!("{}", dest_link);
        if dest_link != String::new() {
            source_link = String::new();
            dest_link = String::new();
        }
    }
}
