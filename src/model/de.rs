use serde::Deserialize;

pub fn bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    u8::deserialize(deserializer).map(|x| x == 1)
}

pub fn rain_snow_limit<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum RainShowLimit {
        #[allow(dead_code)]
        Desc(Option<String>),
        Value(u32),
    }

    impl From<RainShowLimit> for Option<u32> {
        fn from(value: RainShowLimit) -> Self {
            match value {
                RainShowLimit::Desc(_) => None,
                RainShowLimit::Value(v) => Some(v),
            }
        }
    }

    RainShowLimit::deserialize(deserializer).map(Into::into)
}

pub fn timestamp<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;

    chrono::DateTime::from_timestamp(timestamp, 0)
        .map(|x| x.naive_local())
        .ok_or_else(|| serde::de::Error::custom(format!("Invalid timestamp: {timestamp}")))
}

pub fn weather<'de, D>(deserializer: D) -> Result<Option<crate::model::forecast::Weather>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    struct WeatherNull {
        icon: Option<String>,
        desc: Option<String>,
    }

    let Some(weather) = Option::<WeatherNull>::deserialize(deserializer)? else {
        return Ok(None);
    };

    if weather.icon.is_none() && weather.desc.is_none() {
        Ok(None)
    } else {
        Ok(Some(crate::model::forecast::Weather {
            icon: weather.icon.unwrap(),
            desc: weather.desc.unwrap(),
        }))
    }
}
