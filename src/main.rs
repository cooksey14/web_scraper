#[tokio::main]

async fn main() {
    get_html().await;
}

async fn get_html() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://stacoverflow.com/search?q=rust").await?.text().await?;
    println!("{:#?}", html);
    Ok(())
}
