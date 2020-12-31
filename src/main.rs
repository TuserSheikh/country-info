mod country;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use country::Country;

fn main() {
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

    println!("country name: {}", country_name);
}
