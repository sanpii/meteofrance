#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Forecast {
    pub position: super::Position,
    pub updated_on: u32,
    pub daily_forecast: Vec<DailyData>,
    pub forecast: Vec<Data>,
    #[serde(default)]
    pub probability_forecast: Vec<ProbabilityData>,
}

impl Forecast {
    pub fn today_forecast(&self) -> Option<&DailyData> {
        self.daily_forecast.get(0)
    }

    pub fn nearest_forecast(&self) -> Option<&Data> {
        todo!()
    }

    pub fn current_forecast(&self) -> Option<&Data> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DailyData {
    pub dt: u32,
    #[serde(rename = "T")]
    pub temperature: DailyTemperature,
    pub humidity: Humidity,
    pub precipitation: std::collections::BTreeMap<String, Option<f32>>,
    pub uv: Option<u32>,
    #[serde(rename = "weather12H")]
    pub weather_12h: Option<Weather>,
    pub sun: Sun,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Data {
    pub dt: u32,
    #[serde(rename = "T")]
    pub temperature: Temperature,
    pub humidity: Option<u8>,
    pub sea_level: Option<f32>,
    pub wind: Wind,
    pub rain: std::collections::BTreeMap<String, Option<f32>>,
    #[serde(default)]
    pub snow: std::collections::BTreeMap<String, f32>,
    pub iso0: Option<i32>,
    #[serde(
        rename = "rain snow limit",
        deserialize_with = "super::de::rain_snow_limit"
    )]
    pub rain_snow_limit: Option<u32>,
    pub clouds: Option<u32>,
    pub weather: Weather,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ProbabilityData {
    pub dt: u32,
    pub rain: std::collections::BTreeMap<String, Option<f32>>,
    pub snow: std::collections::BTreeMap<String, Option<u16>>,
    #[serde(deserialize_with = "super::de::bool")]
    pub freezing: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DailyTemperature {
    min: Option<f32>,
    max: Option<f32>,
    sea: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Humidity {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Temperature {
    value: Option<f32>,
    windchill: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Sun {
    #[serde(deserialize_with = "super::de::timestamp")]
    rise: chrono::NaiveDateTime,
    #[serde(deserialize_with = "super::de::timestamp")]
    set: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Wind {
    speed: Option<u8>,
    gust: Option<u8>,
    direction: Option<i16>,
    icon: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Weather {
    icon: Option<String>,
    desc: Option<String>,
}

#[cfg(test)]
mod test {
    #[test]
    fn france() -> crate::Result {
        let client = crate::Client::default();

        let forecast = client.forecast(48.8075, 2.24028, None);
        dbg!(&forecast);
        assert!(dbg!(forecast).is_ok());

        Ok(())
    }

    #[test]
    fn world() -> crate::Result {
        let client = crate::Client::default();

        let forecast = client.forecast(45.5016889, 73.567256, None);
        assert!(dbg!(forecast).is_ok());

        Ok(())
    }

    #[test]
    fn place() -> crate::Result {
        let client = crate::Client::default();

        let place = crate::model::Place {
            insee: Some("74080".to_string()),
            name: "La Clusaz".to_string(),
            lat: 45.90417,
            lon: 6.42306,
            country: "FR".to_string(),
            admin: "Rhône-Alpes".to_string(),
            admin2: Some("74".to_string()),
            post_code: Some("74220".to_string()),
        };

        let forecast = client.forecast_for_place(&place, None);
        assert!(dbg!(forecast).is_ok());

        Ok(())
    }
}
