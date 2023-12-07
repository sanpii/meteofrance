pub mod dictionary;

mod forecast;
mod observation;
mod phenomenoms;
mod place;
mod rain;
mod warning;

pub use dictionary::Dictionary;
pub use forecast::Forecast;
pub use observation::Observation;
pub use phenomenoms::Phenomenoms;
pub use place::Place;
pub use rain::Rain;
pub use warning::Warning;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Position {
    pub lat: f32,
    pub lon: f32,
    pub alti: i32,
    pub name: String,
    pub country: String,
    pub dept: Option<String>,
    #[serde(default)]
    pub rain_product_available: u8,
    pub timezone: String,
    pub insee: Option<String>,
    pub bulletin_cote: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Error {
    pub error: u32,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Picture1 {}

#[derive(Clone, Debug, PartialEq)]
pub struct Picture {
    pub image_url: String,
    pub description: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn picture_of_the_day() -> crate::Result {
        let client = crate::Client::default();
        let picture = client.picture_of_the_day(None)?;

        assert_eq!(picture.image_url, "https://webservice.meteofrance.com/v2/report?domain=france&report_type=observation&report_subtype=image%20du%20jour&format=jpg&token=__Wj7dVSTjV9YGu1guveLyDq0g7S7TfTjaHBTPTpO0kj8__");

        Ok(())
    }
}
