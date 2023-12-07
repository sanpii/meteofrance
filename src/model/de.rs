pub fn rain_snow_limit<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum RainShowLimit {
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
    use serde::Deserialize;

    let timestamp = i64::deserialize(deserializer)?;

    chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .ok_or_else(|| serde::de::Error::custom(format!("Invalid timestamp: {timestamp}")))
}
