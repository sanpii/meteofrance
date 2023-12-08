#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Observation {
    pub update_time: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub geometry: Geometry,
    properties: Properties,
}

impl Observation {
    pub fn timezone(&self) -> &str {
        self.properties.timezone.as_str()
    }

    pub fn time(&self) -> &str {
        self.properties.gridded.time.as_str()
    }

    pub fn temperature(&self) -> f32 {
        self.properties.gridded.temperature
    }

    pub fn wind_speed(&self) -> f32 {
        self.properties.gridded.wind_speed
    }

    pub fn wind_direction(&self) -> i32 {
        self.properties.gridded.wind_direction
    }

    pub fn wind_icon(&self) -> &str {
        self.properties.gridded.wind_icon.as_str()
    }

    pub fn weather_icon(&self) -> &str {
        self.properties.gridded.weather_icon.as_str()
    }

    pub fn weather_description(&self) -> &str {
        self.properties.gridded.weather_description.as_str()
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Geometry {
    #[serde(rename = "type")]
    pub ty: String,
    pub coordinates: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Properties {
    timezone: String,
    gridded: Gridded,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Gridded {
    time: String,
    #[serde(rename = "T")]
    temperature: f32,
    wind_speed: f32,
    wind_direction: i32,
    wind_icon: String,
    weather_icon: String,
    weather_description: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn france() -> crate::Result {
        let client = crate::Client::default();

        let rain = client.observation(48.8075, 2.24028);
        assert!(dbg!(rain).is_ok());

        Ok(())
    }

    #[test]
    fn world() -> crate::Result {
        let client = crate::Client::default();

        let rain = client.observation(45.5016889, 73.567256);
        assert!(rain.is_err());

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

        let rain = client.observation_for_place(&place);
        assert!(dbg!(rain).is_ok());

        Ok(())
    }
}
