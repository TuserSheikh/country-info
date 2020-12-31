mod country;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use country::Country;
use hyper::Uri;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("country")
                .required(true)
                .validator(Country::correct_country_name),
        )
        .get_matches();

    let country_name = app.value_of("country").unwrap().to_lowercase();
    let url = format!("http://restcountries.eu/rest/v2/name/{}", country_name);
    let uri = url.parse::<Uri>()?;

    let countries = Country::fetch_json(uri).await?;

    println!("{:?}", countries);

    Ok(())
}
