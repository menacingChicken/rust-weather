use super::{MyError, GeoLocation, Settings};

pub fn set_city(s: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let vals: Vec<&str> = s.split(",").collect();

    if vals.len() != 2
    {
        return Err(MyError::new("Invalid parameter: city,state"));
    }

    println!("Setting city to: {}", vals[0]);

    let location = GeoLocation::from_city_state(vals[0], vals[1])?;

    // Save location

    let mut settings = Settings::load_or_create("settings.json");
    settings.city_name = Some(vals[0].to_owned());
    settings.state = Some(vals[1].to_owned());
    settings.location = Some(location);
    settings.to_file("settings.json")?;

    Ok(())
}
