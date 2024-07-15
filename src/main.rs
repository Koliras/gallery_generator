use std::fs;

const ENGLISH_WIKI_URL_BASE: &str = "https://residentevil.fandom.com/wiki/";
const ENGLISH_API_BASE_URL: &str = "https://residentevil.fandom.com/api.php?action=query&prop=revisions&rvprop=content&format=json&rvslots=main";

#[tokio::main]
async fn main() {
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

        let eng_api_url = format!("{}&titles={}%2Fgallery", ENGLISH_API_BASE_URL, source_title);

        let res = reqwest::get(&eng_api_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let strings =
            res[res.chars().position(|c| c == '*').unwrap_or(0) + 4..res.len() - 9].split("\\n");

        let mut in_gallery = false;

        let converted_strings: Vec<String> = strings
            .map(|s| {
                if s.contains("<gallery") {
                    in_gallery = true;
                    return s.to_string();
                } else if s.contains("</gallery>") {
                    in_gallery = false;
                    return s.to_string();
                }
                if !in_gallery {
                    return s.to_string();
                }

                return format_image_string(s);
            })
            .collect();

        let content = converted_strings.join("\\n");

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

fn format_image_string(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }

    if s.contains('|') {
        let image_part_end = s.chars().position(|c| c == '|').unwrap();
        let mut image = s[0..image_part_end].to_string();
        let dot = image.rfind('.').unwrap();
        image = format!("{}.webp", &image[0..dot]);
        return format!("{}{}", image, &s[image_part_end..]);
    }

    let dot = s.rfind('.').unwrap();
    return format!("{}.webp", &s[..dot]);
}
