mod weather;
mod settings;
mod error;
mod cli;

use weather::{GeoLocation, Weather};
use structopt::StructOpt;
use settings::Settings;
use error::MyError;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    pub set_city: Option<String>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    if let Some(name) = opt.set_city
    {
        cli::set_city(&name)?;
    }

    let settings = Settings::from_file("settings.json")?;

    if let Some(loc) = settings.location
    {
        let weather = Weather::fetch_from_geo_location(&loc)?;
        println!("Current weather: {:?}", weather);
    }
    else
    {
        println!("No city set. Use --set-city=Auburn,CA to set a city");
    }

    Ok(())
}
