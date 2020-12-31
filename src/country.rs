use bytes::Buf as _;
use hyper::{Client, Uri};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Country {
    name: String,
    capital: String,
    region: String,
    area: f64,
    flag: String,
}

impl Country {
    pub fn correct_country_name(name: String) -> Result<(), String> {
        let countries = ["bangladesh", "canada"];

        match countries.iter().find(|&&c| c == name.to_lowercase()) {
            Some(_) => Ok(()),
            None => Err(format!("{} is not a valid country name", name)),
        }
    }

    pub async fn fetch_json(
        uri: Uri,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let response = client.get(uri).await?;
        let body = hyper::body::aggregate(response).await?;
        let countries = serde_json::from_reader(body.reader())?;

        Ok(countries)
    }
}
