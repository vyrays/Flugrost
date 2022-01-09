use serde_json::{Value};

#[derive(Debug)]
pub struct Weather {
    pub label: String,
    pub temp: String,
    pub temp_felt: String,
    pub wind_min: String,
    pub wind_max: String,
    pub time: String,
    pub chanceofsnow: String,
    pub chanceofrain: String,
    pub precipitation: String,
}

impl From<&Value> for Weather {
    fn from(val: &Value) -> Self {
        let mut time: String = val["time"].to_string().replace("\"", "");
        match time.len() {
            3 => {
                time.insert(1, ':');
            }
            4 => {
                time.insert(2, ':');
            }
            _ => {}
        }

        Weather {
            label: val["weatherDesc"][0]["value"].to_string().replace("\"", ""),
            temp: val["tempC"].to_string().replace("\"", ""),
            temp_felt: val["FeelsLikeC"].to_string().replace("\"", ""),
            wind_min: val["windspeedKmph"].to_string().replace("\"", ""),
            wind_max: val["WindGustKmph"].to_string().replace("\"", ""),
            time,
            chanceofsnow: val["chanceofsnow"].to_string().replace("\"", ""),
            chanceofrain: val["chanceofrain"].to_string().replace("\"", ""),
            precipitation: val["precipMM"].to_string().replace("\"", ""),
        }
    }
}
