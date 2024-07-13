use std::fs;

const ENGLISH_WIKI_URL_BASE: &str = "https://residentevil.fandom.com/wiki/";
const UKRAINIAN_WIKI_URL_BASE: &str = "https://resident-evil.fandom.com/uk/wiki/";

const ENGLISH_API_BASE_URL: &str = "https://residentevil.fandom.com/api.php?action=query&prop=revisions&rvprop=content&format=json&rvslots=main";
const UKRAINIAN_API_BASE_URL: &str = "https://resident-evil.fandom.com/uk/api.php?action=query&prop=revisions&rvprop=content&format=json&rvslots=main";

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

        let eng_api_url = format!("{}&titles={}%2Fgallery", ENGLISH_API_BASE_URL, source_title);
        let ukr_api_url = format!("{}&titles={}%2Fгалерея", UKRAINIAN_API_BASE_URL, dest_title);

        println!("eng api url: {}", eng_api_url);
        println!("ukr api url: {}", ukr_api_url);

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
