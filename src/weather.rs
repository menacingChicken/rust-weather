use super::MyError;

const API_KEY: &str = env!("API_KEY");

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GeoLocation {
    lat: f32,
    lon: f32
}

impl GeoLocation {
    pub fn from_city_state(name: &str, state: &str) -> Result<GeoLocation, Box<dyn std::error::Error>> {
        let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={city},{state},{country}&limit={limit}&appid={key}",
            city=name,
            state=state,
            country="US",
            limit=1,
            key=API_KEY
        );

        let mut locations: Vec<GeoLocation> = reqwest::blocking::get(url)?.json()?;

        match locations.pop()
        {
            Some(v) => Ok(v),
            None => Err(MyError::new("could not resolve location"))
        }
    }
}

#[derive(Debug)]
pub struct Weather {
    desc: String,
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32
}

#[derive(serde::Deserialize)]
struct _CurrentWeather {
    weather: Vec<_Weather>,
    main: _Main
}

#[derive(serde::Deserialize)]
struct _Weather {
    description: String,
}

#[derive(serde::Deserialize)]
struct _Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32
}

impl std::convert::From<_CurrentWeather> for Weather {
    fn from(current: _CurrentWeather) -> Weather {
        Weather {
            desc: current.weather[0].description.to_owned(),
            temp: current.main.temp,
            temp_max: current.main.temp_max,
            temp_min: current.main.temp_min,
            feels_like: current.main.feels_like
        }
    }
}


impl Weather {
    pub fn fetch_from_geo_location(loc: &GeoLocation) -> Result<Weather, Box<dyn std::error::Error>> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={key}&units=imperial",
            lat=loc.lat,
            lon=loc.lon,
            key=API_KEY
        );

        let json = reqwest::blocking::get(url)?.json::<_CurrentWeather>()?.into();

        Ok(json)
    }
}