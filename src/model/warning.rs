#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Warning {
    pub update_time: u32,
    pub end_validity_time: u32,
    pub domain_id: String,
    pub color_max: u16,
    pub timelaps: Vec<Timelap>,
    pub phenomenons_items: Vec<super::phenomenoms::Color>,
    pub advices: Option<String>,
    pub consequences: Option<String>,
    pub max_count_items: Option<usize>,
    pub comments: Comments,
    pub text: Option<String>,
    pub text_avalanche: Option<String>,
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
    pub begin_time: u32,
    pub end_time: u32,
    pub color_id: u16,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Comments {
    pub title: String,
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

        assert_eq!(thumbnail, "https://webservice.meteofrance.com/v3/warning/thumbnail?&token=__Wj7dVSTjV9YGu1guveLyDq0g7S7TfTjaHBTPTpO0kj8__&domain=france");
    }
}
