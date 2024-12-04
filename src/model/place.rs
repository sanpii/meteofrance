#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Place {
    pub insee: Option<String>,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub admin: String,
    pub admin2: Option<String>,
    #[serde(rename = "postCode")]
    pub post_code: Option<String>,
}

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ", self.name, self.admin)?;
        if self.country == "FR" {
            write!(f, "({}) ", self.admin2.clone().unwrap_or_default())?;
        }
        write!(f, "- {}", self.country)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get() -> crate::Result {
        let client = crate::Client::default();
        let places = client.search_places("montreal", None, None)?;

        assert!(!places.is_empty());

        assert_eq!(
            places[0],
            crate::model::Place {
                insee: Some("11254".to_string()),
                name: "Montréal".to_string(),
                lat: 43.2,
                lon: 2.14083,
                country: "FR".to_string(),
                admin: "Languedoc-Roussillon".to_string(),
                admin2: Some("11".to_string()),
                post_code: Some("11290".to_string()),
            }
        );

        Ok(())
    }

    #[test]
    fn with_gps() -> crate::Result {
        let client = crate::Client::default();
        let places = client.search_places("montreal", Some(45.50884), Some(-73.58))?;

        assert!(!places.is_empty());

        assert_eq!(
            places[0],
            crate::model::Place {
                insee: None,
                name: "Montréal".to_string(),
                lat: 45.50884,
                lon: -73.58781,
                country: "CA".to_string(),
                admin: "Quebec".to_string(),
                admin2: Some("06".to_string()),
                post_code: None,
            }
        );

        Ok(())
    }

    #[test]
    fn not_found() -> crate::Result {
        let client = crate::Client::default();
        let places = client.search_places("sqdmfkjdsmkf", None, None)?;

        assert!(places.is_empty());

        Ok(())
    }

    #[test]
    fn print() -> crate::Result {
        let client = crate::Client::default();

        let places_in_france = client.search_places("Montréal", None, None)?;
        let places_not_in_france =
            client.search_places("Montréal", Some(45.50884), Some(-73.58))?;

        assert_eq!(
            places_in_france[0].to_string(),
            "Montréal - Languedoc-Roussillon (11) - FR"
        );
        assert_eq!(
            places_not_in_france[0].to_string(),
            "Montréal - Quebec - CA"
        );

        Ok(())
    }
}
