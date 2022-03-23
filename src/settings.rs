use super::GeoLocation;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub city_name: Option<String>,
    pub state: Option<String>,
    pub location: Option<GeoLocation>
}

impl Settings {
    pub fn new() -> Settings
    {
        Settings {
            city_name: None,
            state: None,
            location: None
        }
    }

    pub fn from_file(path: &str) -> Result<Settings, Box<dyn std::error::Error>>
    {
        Ok(
            serde_json::from_reader(
                std::io::BufReader::new(
                    std::fs::File::open(path)?
                )
            )?
        )
    }

    pub fn load_or_create(path: &str) -> Settings
    {
        match Settings::from_file(path)
        {
            Ok(s) => s,
            Err(_) => Settings::new()
        }
    }

    pub fn to_file(self, path: &str) -> Result<(), Box<dyn std::error::Error>>
    {
        let f = std::fs::File::create("settings.json")?;
        serde_json::to_writer(&f, &self)?;
        Ok(())
    }
}
