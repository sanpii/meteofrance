#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Warning {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub update_time: chrono::NaiveDateTime,
    #[serde(deserialize_with = "super::de::timestamp")]
    pub end_validity_time: chrono::NaiveDateTime,
    pub domain_id: String,
    pub color_max: u16,
    pub timelaps: Vec<Timelap>,
    pub phenomenons_items: Vec<super::phenomenoms::Color>,
    pub advices: Option<String>,
    pub consequences: Option<String>,
    pub max_count_items: Option<usize>,
    pub comments: Comments,
    pub text: Option<Text>,
    pub text_avalanche: Option<Text>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Timelap {
    pub phenomenon_id: String,
    pub timelaps_items: Vec<TimelapsItem>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct TimelapsItem {
    #[serde(deserialize_with = "super::de::timestamp")]
    pub begin_time: chrono::NaiveDateTime,
    #[serde(deserialize_with = "super::de::timestamp")]
    pub end_time: chrono::NaiveDateTime,
    pub color_id: u16,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Comments {
    pub title: String,
    pub text: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Text {
    pub bloc_title: String,
    pub text_bloc_item: Vec<TextBloc>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct TextBloc {
    pub type_name: String,
    pub text_items: Vec<TextItem>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct TextItem {
    pub type_code: String,
    pub hazard_code: Option<String>,
    pub hazard_name: String,
    pub term_items: Vec<TermItem>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct TermItem {
    pub term_names: String,
    pub start_time: String,
    pub end_time: String,
    pub risk_name: String,
    pub subdivision_text: Vec<SubdivisionText>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SubdivisionText {
    pub underline_text: String,
    pub text: Vec<String>,
}

#[cfg(test)]
mod test {
    #[test]
    fn full() -> crate::Result {
        let client = crate::Client::default();
        let warning = client.warning_full("32", None);
        assert!(dbg!(warning).is_ok());

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
            warning: &'a crate::model::Warning,
            id: &str,
        ) -> Option<&'a crate::model::phenomenoms::Color> {
            warning
                .phenomenons_items
                .iter()
                .find(|x| x.phenomenon_id == id)
        }

        for (dep, coastal, avalanche) in tests {
            let warning = client.warning_full(dep, Some(true))?;

            assert_eq!(phenomenon_by_id(&warning, "8").is_some(), avalanche);
            assert_eq!(phenomenon_by_id(&warning, "9").is_some(), coastal);
        }

        Ok(())
    }

    #[test]
    fn thumbnail() {
        let client = crate::Client::default();
        let thumbnail = client.warning_thumbnail(None);

        assert_eq!(
            thumbnail,
            "https://webservice.meteofrance.com/v3/warning/thumbnail?&token=__Wj7dVSTjV9YGu1guveLyDq0g7S7TfTjaHBTPTpO0kj8__&domain=france"
        );
    }
}
