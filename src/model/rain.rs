#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Rain {
    pub position: super::Position,
    pub updated_on: u32,
    pub forecast: Vec<Forecast>,
    pub quality: u8,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Forecast {
    pub dt: u32,
    pub rain: u8,
    pub desc: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn get() -> crate::Result {
        let client = crate::Client::default();
        let rain = client.rain(48.8075, 2.24028, None);

        assert!(dbg!(rain).is_ok());

        Ok(())
    }

    #[test]
    fn not_covered() -> crate::Result {
        let client = crate::Client::default();
        let rain = client.rain(45.508, -73.58, None);

        assert!(rain.is_err());

        Ok(())
    }
}
