extern crate select;
use select::document::Document;
use select::predicate::Name;
use std::io::Cursor;
#[tokio::main]

async fn main() {
    let html_data:std::string::String = get_html().await.unwrap();
    //println!("{:#?}", html_data);
    let cursor:std::io::Cursor<std::string::String> = Cursor::new(html_data);
    //println!("{:#?}", cursor);
    let document = Document::from_read(cursor);
    //println!("{:#?}", document);
    let found = document
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    //println!("{}", found);
}

async fn get_html() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let html = reqwest::get("https://stackoverflow.com/search?q=rust")
        .await?
        .text()
        .await?;
    Ok(html)
}
