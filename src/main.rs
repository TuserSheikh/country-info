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
            Arg::with_name("country_iso_code")
                .required(true)
                .validator(Country::correct_country_name),
        )
        .get_matches();

    let country_name = app.value_of("country_iso_code").unwrap().to_lowercase();
    let url = format!("http://restcountries.eu/rest/v2/alpha/{}", country_name);
    let uri = url.parse::<Uri>()?;

    let country = Country::fetch_json(uri).await?;

    println!("   Country Name: {}", country.name());
    println!("   Capital City: {}", country.capital());
    println!("         Rigion: {}", country.region());
    println!("          Areas: {} Square Kilometers", country.area());

    for (i, c) in country.currencies().iter().enumerate() {
        match i {
            0 => println!("     Currencies: {}", c),
            _ => println!("               - {}", c),
        }
    }

    for (i, c) in country.languages().iter().enumerate() {
        match i {
            0 => println!("      Languages: {}", c),
            _ => println!("               - {}", c),
        }
    }

    Ok(())
}
