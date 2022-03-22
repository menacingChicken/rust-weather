mod weather;

use weather::{GeoLocation, Weather};
use structopt::StructOpt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Settings {
    city_name: Option<String>,
    state: Option<String>,
    location: Option<GeoLocation>
}

impl Settings {
    fn new() -> Settings
    {
        Settings {
            city_name: None,
            state: None,
            location: None
        }
    }

    fn from_file(path: &str) -> Result<Settings, Box<dyn std::error::Error>>
    {
        Ok(
            serde_json::from_reader(
                std::io::BufReader::new(
                    std::fs::File::open(path)?
                )
            )?
        )
    }

    fn load_or_create(path: &str) -> Settings
    {
        match Settings::from_file(path)
        {
            Ok(s) => s,
            Err(_) => Settings::new()
        }
    }

    fn to_file(self, path: &str) -> Result<(), Box<dyn std::error::Error>>
    {
        let f = std::fs::File::create("settings.json")?;
        serde_json::to_writer(&f, &self)?;
        Ok(())
    }
}

#[derive(Debug)]
struct MyError {
    msg: String
}

impl std::fmt::Display for MyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(fmt, "{}", self.msg)
    }
}

impl std::error::Error for MyError {

}

impl MyError {
    fn new(msg: &str) -> Box<dyn std::error::Error>
    {
        Box::new(MyError {
            msg: msg.to_owned()
        })
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    set_city: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    if let Some(name) = opt.set_city
    {
        set_city(&name)?;
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

fn set_city(s: &str) -> Result<(), Box<dyn std::error::Error>>
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