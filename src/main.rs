#[tokio::main]

async fn main() {
   let html_data = get_html().await;
   println!("{:#?}", html_data);
}

async fn get_html() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let html = reqwest::get("https://stacoverflow.com/search?q=rust").await?.text().await?;
    //println!("{:#?}", html);
    Ok(html)
}
