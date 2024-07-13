use std::fs;

const ENGLISH_WIKI_URL_BASE: &str = "https://residentevil.fandom.com/wiki/";
const UKRAINIAN_WIKI_URL_BASE: &str = "https://resident-evil.fandom.com/uk/wiki/";

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

        let source_title = extract_article_title(&source_link, ENGLISH_WIKI_URL_BASE);
        let dest_title = extract_article_title(&dest_link, UKRAINIAN_WIKI_URL_BASE);

        println!("first link: ");
        println!("{}", source_link);
        println!("second link: ");
        println!("{}", dest_link);
        println!("source title: ");
        println!("{}", source_title);
        println!("dest title: ");
        println!("{}", dest_title);

        if dest_link != String::new() {
            source_link = String::new();
            dest_link = String::new();
        }
    }
}

fn extract_article_title(link: &String, base_url: &str) -> String {
    let mut url = link[base_url.len()..].to_string();
    url = url[0..url
        .chars()
        .position(|c| c == '/' || c == '?')
        .unwrap_or_else(|| return url.len())]
        .to_string();
    return url;
}
