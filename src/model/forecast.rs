#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Forecast {
    pub position: super::Position,
    #[serde(deserialize_with = "super::de::timestamp")]
    pub updated_on: jiff::civil::DateTime,
    pub daily_forecast: Vec<DailyData>,
    pub forecast: Vec<Data>,
    #[serde(default)]
    pub probability_forecast: Vec<ProbabilityData>,
}

impl Forecast {
    /**
     * Return the forecast for today.
     */
    #[must_use]
    pub fn today_forecast(&self) -> Option<&DailyData> {
        self.daily_forecast.first()
    }

    /**
     * Return the nearest hourly forecast.
     */
    #[must_use]
    pub fn nearest_forecast(&self) -> Option<&Data> {
        let now = jiff::Zoned::now().datetime();

        self.forecast.iter().min_by(|a, b| {
            let da = (a.dt - now).abs();
            let db = (b.dt - now).abs();

            da.compare(jiff::SpanCompare::from(&db).days_are_24_hours())
                .unwrap()
        })
    }

    /**
     * Return the forecast of the current hour.
     */
    #[must_use]
    pub fn current_forecast(&self) -> Option<&Data> {
        let current_hour = jiff::Zoned::now()
            .datetime()
            .with()
            .minute(0)
            .second(0)
            .nanosecond(0)
            .build()
            .unwrap();

        self.forecast
            .iter()
            .find(|x| x.dt == current_hour)
            .or_else(|| self.nearest_forecast())
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DailyData {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub dt: jiff::civil::DateTime,
    #[serde(rename = "T")]
    pub temperature: DailyTemperature,
    pub humidity: Humidity,
    pub precipitation: std::collections::BTreeMap<String, Option<f32>>,
    pub uv: Option<u32>,
    #[serde(
        default,
        rename = "weather12H",
        deserialize_with = "super::de::weather"
    )]
    pub weather_12h: Option<Weather>,
    pub sun: Sun,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Data {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub dt: jiff::civil::DateTime,
    #[serde(rename = "T")]
    pub temperature: Temperature,
    pub humidity: Option<u8>,
    pub sea_level: Option<f32>,
    pub wind: Wind,
    pub rain: std::collections::BTreeMap<String, Option<f32>>,
    #[serde(default)]
    pub snow: std::collections::BTreeMap<String, Option<f32>>,
    pub iso0: Option<i32>,
    #[serde(
        rename = "rain snow limit",
        deserialize_with = "super::de::rain_snow_limit"
    )]
    pub rain_snow_limit: Option<u32>,
    pub clouds: Option<u32>,
    #[serde(deserialize_with = "super::de::weather")]
    pub weather: Option<Weather>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ProbabilityData {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub dt: jiff::civil::DateTime,
    pub rain: std::collections::BTreeMap<String, Option<f32>>,
    pub snow: std::collections::BTreeMap<String, Option<u16>>,
    #[serde(deserialize_with = "super::de::bool")]
    pub freezing: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DailyTemperature {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub sea: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Humidity {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Temperature {
    pub value: Option<f32>,
    pub windchill: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Sun {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub rise: jiff::civil::DateTime,
    #[serde(deserialize_with = "super::de::timestamp")]
    pub set: jiff::civil::DateTime,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Wind {
    pub speed: Option<u8>,
    pub gust: Option<u8>,
    pub direction: Option<i16>,
    pub icon: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Weather {
    pub icon: String,
    pub desc: String,
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn france() -> crate::Result {
        let client = crate::Client::default();

        let forecast = client.forecast(48.8075, 2.24028).await;
        std::assert_matches!(forecast, Ok(_));

        Ok(())
    }

    #[tokio::test]
    async fn world() -> crate::Result {
        let client = crate::Client::default();

        let forecast = client.forecast(45.501_689, 73.567_256).await?;

        assert_eq!(
            forecast.current_forecast().unwrap().dt,
            forecast.nearest_forecast().unwrap().dt,
        );

        assert_eq!(
            forecast.today_forecast().unwrap().dt,
            forecast.daily_forecast[0].dt,
        );

        Ok(())
    }

    #[tokio::test]
    async fn place() -> crate::Result {
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

        let forecast = client.forecast_for_place(&place).await;
        std::assert_matches!(forecast, Ok(_));

        Ok(())
    }
}
