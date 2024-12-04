#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Phenomenoms {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub update_time: chrono::NaiveDateTime,
    #[serde(deserialize_with = "super::de::timestamp")]
    pub end_validity_time: chrono::NaiveDateTime,
    pub domain_id: String,
    pub phenomenons_max_colors: Vec<Color>,
}

impl Phenomenoms {
    pub fn domain_max_color(&self) -> u16 {
        self.phenomenons_max_colors
            .iter()
            .map(|x| x.phenomenon_max_color_id)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Color {
    pub phenomenon_id: String,
    pub phenomenon_max_color_id: u16,
}

#[cfg(test)]
mod test {
    #[test]
    fn get() -> crate::Result {
        let json = r#"
{
    "update_time": 1591279200,
    "end_validity_time": 1591365600,
    "domain_id": "32",
    "phenomenons_max_colors": [
        {"phenomenon_id": "6", "phenomenon_max_color_id": 1},
        {"phenomenon_id": "4", "phenomenon_max_color_id": 1},
        {"phenomenon_id": "5", "phenomenon_max_color_id": 3},
        {"phenomenon_id": "2", "phenomenon_max_color_id": 1},
        {"phenomenon_id": "1", "phenomenon_max_color_id": 1},
        {"phenomenon_id": "3", "phenomenon_max_color_id": 2}
    ]
}"#;

        let phenomenoms: crate::model::Phenomenoms = serde_json::from_str(json).unwrap();
        assert_eq!(phenomenoms.domain_max_color(), 3);

        let client = crate::Client::default();
        let phenomenoms = client.warning_current_phenomenoms("32", Some(1), None);
        assert!(dbg!(phenomenoms).is_ok());

        Ok(())
    }

    #[test]
    fn with_coastal_bulletin() -> crate::Result {
        let client = crate::Client::default();

        let tests = [
            ("13", true, false),
            ("32", false, false),
            ("74", false, true),
        ];

        fn phenomenon_by_id<'a>(
            phenomenoms: &'a crate::model::Phenomenoms,
            id: &str,
        ) -> Option<&'a crate::model::phenomenoms::Color> {
            phenomenoms
                .phenomenons_max_colors
                .iter()
                .find(|x| x.phenomenon_id == id)
        }

        for (dep, coastal, avalanche) in tests {
            let phenomenoms = client.warning_current_phenomenoms(dep, Some(1), Some(true))?;

            assert_eq!(phenomenon_by_id(&phenomenoms, "8").is_some(), avalanche);
            assert_eq!(phenomenon_by_id(&phenomenoms, "9").is_some(), coastal);
        }

        Ok(())
    }
}
