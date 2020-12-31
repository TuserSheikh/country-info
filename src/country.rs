pub struct Country {
    name: String,
    capital: String,
    region: String,
    area: String,
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
}
