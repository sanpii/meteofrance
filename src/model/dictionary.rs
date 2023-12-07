#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Dictionary {
    pub phenomenons: Vec<Phenomenon>,
    pub colors: Vec<Color>,
}

impl Dictionary {
    pub fn color_by_id(&self, id: u16) -> Option<&Color> {
        self.colors.iter().find(|x| x.id == id)
    }

    pub fn color_name_by_id(&self, id: u16) -> Option<&str> {
        self.colors
            .iter()
            .find(|x| x.id == id)
            .map(|x| x.name.as_str())
    }

    pub fn phenomenon_by_id(&self, id: u16) -> Option<&Phenomenon> {
        self.phenomenons.iter().find(|x| x.id == id)
    }

    pub fn phenomenon_name_by_id(&self, id: u16) -> Option<&str> {
        self.phenomenons
            .iter()
            .find(|x| x.id == id)
            .map(|x| x.name.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Phenomenon {
    pub id: u16,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Color {
    pub id: u16,
    pub level: u8,
    pub name: String,
    #[serde(rename = "hexaCode")]
    pub hexa_code: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn get() -> crate::Result {
        let client = crate::Client::default();
        let dictionary = client.warning_dictionary(None);

        assert!(dbg!(dictionary).is_ok());

        Ok(())
    }

    #[test]
    fn phenomenon_by_id() -> crate::Result {
        let client = crate::Client::default();
        let dictionary = client.warning_dictionary(None)?;

        assert_eq!(
            dictionary.phenomenon_by_id(1),
            Some(&crate::model::dictionary::Phenomenon {
                id: 1,
                name: "Vent".to_string(),
            })
        );

        Ok(())
    }

    #[test]
    fn phenomenon_name_by_id() -> crate::Result {
        let client = crate::Client::default();
        let dictionary = client.warning_dictionary(None)?;

        assert_eq!(
            dictionary.phenomenon_name_by_id(2),
            Some("Pluie-inondation")
        );

        Ok(())
    }

    #[test]
    fn color_by_id() -> crate::Result {
        let client = crate::Client::default();
        let dictionary = client.warning_dictionary(None)?;

        assert_eq!(
            dictionary.color_by_id(1),
            Some(&crate::model::dictionary::Color {
                id: 1,
                level: 1,
                name: "vert".to_string(),
                hexa_code: "#31aa35".to_string(),
            })
        );

        Ok(())
    }

    #[test]
    fn color_name_by_id() -> crate::Result {
        let client = crate::Client::default();
        let dictionary = client.warning_dictionary(None)?;

        assert_eq!(dictionary.color_name_by_id(1), Some("vert"));

        Ok(())
    }
}
