use agnus_daily_error::AgnusDailyError;

pub async fn fetch(url: &str) -> Result<String, AgnusDailyError> {
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        println!(
            "Failed to fetch the URL: {}, status {}",
            url,
            response.status()
        );
        Err(AgnusDailyError {
            title: String::from("rewest is retarded"),
            description: format!("response status {}", response.status()),
        })
    }
}
