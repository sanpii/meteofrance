#[derive(Clone, Debug)]
pub struct Client {
    session: crate::Session,
    language: String,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl Client {
    pub fn new(access_token: Option<&str>, language: Option<&str>) -> Self {
        Self {
            language: language.unwrap_or("fr").to_string(),
            session: crate::Session::new(access_token),
        }
    }

    /**
     * Search the places (cities) linked to a query by name.
     *
     * You can add GPS coordinates in parameter to search places arround a given
     * location.
     */
    pub fn search_places(
        &self,
        search_query: &str,
        latitude: Option<f32>,
        longitude: Option<f32>,
    ) -> crate::Result<Vec<crate::model::Place>> {
        let mut params = std::collections::HashMap::new();
        params.insert("q", search_query.to_string());

        if let Some(latitude) = latitude {
            params.insert("lat", latitude.to_string());
        }
        if let Some(longitude) = longitude {
            params.insert("lon", longitude.to_string());
        }

        self.session.get("places", &params)
    }

    /**
     * Retrieve the weather observation for a given GPS location.
     */
    pub fn observation(
        &self,
        latitude: f32,
        longitude: f32,
    ) -> crate::Result<crate::model::Observation> {
        let mut params = std::collections::HashMap::new();
        params.insert("lon", longitude.to_string());
        params.insert("lat", latitude.to_string());
        params.insert("lang", self.language.clone());

        self.session.get("v2/observation", &params)
    }

    /**
     * Retrieve the weather observation for a given Place instance.
     */
    pub fn observation_for_place(
        &self,
        place: &crate::model::Place,
    ) -> crate::Result<crate::model::Observation> {
        self.observation(place.lat, place.lon)
    }

    /**
     * Retrieve the weather forecast for a given GPS location.
     */
    pub fn forecast(&self, latitude: f32, longitude: f32) -> crate::Result<crate::model::Forecast> {
        let mut params = std::collections::HashMap::new();
        params.insert("lon", longitude.to_string());
        params.insert("lat", latitude.to_string());
        params.insert("lang", self.language.clone());

        self.session.get("forecast", &params)
    }

    /**
     * Retrieve the weather forecast for a given Place instance.
     */
    pub fn forecast_for_place(
        &self,
        place: &crate::model::Place,
    ) -> crate::Result<crate::model::Forecast> {
        self.forecast(place.lat, place.lon)
    }

    /**
     * Retrieve the next 1 hour rain forecast for a given GPS the location.
     */
    pub fn rain(&self, latitude: f32, longitude: f32) -> crate::Result<crate::model::Rain> {
        let mut params = std::collections::HashMap::new();
        params.insert("lon", longitude.to_string());
        params.insert("lat", latitude.to_string());
        params.insert("lang", self.language.clone());

        self.session.get("rain", &params)
    }

    /**
     * Return the current weather phenomenoms (or alerts) for a given domain.
     *
     * - `domain`: could be `france` or any metropolitan France department numbers on
     *     two digits. For some departments you can access an additional bulletin
     *     for coastal phenomenoms. To access it add `10` after the domain id
     *     (example: `1310`).
     * - `depth`: Optional; To be used with domain = 'france'. With depth = 0 the
     *     results will show only natinal sum up of the weather alerts. If
     *     depth = 1, you will have in addition, the bulletin for all metropolitan
     *     France department and Andorre
     * - `with_coastal_bulletin`: Optional; If set to True (default is False), you can
     *     get the basic bulletin and coastal bulletin merged.
     */
    pub fn warning_current_phenomenoms(
        &self,
        domain: &str,
        depth: Option<i32>,
        with_coastal_bulletin: Option<bool>,
    ) -> crate::Result<crate::model::Phenomenoms> {
        let mut params = std::collections::HashMap::new();
        params.insert("domain", domain.to_string());
        params.insert("depth", depth.unwrap_or(0).to_string());

        let mut phenomenoms = self
            .session
            .get::<crate::model::Phenomenoms>("v3/warning/currentphenomenons", &params)?;

        if with_coastal_bulletin == Some(true) && crate::COASTAL_DEPARTMENT_LIST.contains(&domain) {
            let mut coastal =
                self.warning_current_phenomenoms(&format!("{domain}10"), None, Some(false))?;
            phenomenoms
                .phenomenons_max_colors
                .append(&mut coastal.phenomenons_max_colors);
        }

        Ok(phenomenoms)
    }

    /**
     * Retrieve a complete bulletin of the weather phenomenons for a given domain.
     *
     * For a given domain we can access the maximum alert, a timelaps of the alert
     * evolution for the next 24 hours, a list of alerts and other metadatas.
     *
     * - `domain`: could be `france` or any metropolitan France department numbers on
     *      two digits. For some departments you can access an additional bulletin
     *      for coastal phenomenoms. To access it add `10` after the domain id
     *      (example: `1310`).
     * - `with_coastal_bulletin`: Optional; If set to True (default is False), you can
     *      get the basic bulletin and coastal bulletin merged.
     */
    pub fn warning_full(
        &self,
        domain: &str,
        with_coastal_bulletin: Option<bool>,
    ) -> crate::Result<crate::model::Warning> {
        let mut params = std::collections::HashMap::new();
        params.insert("domain", domain.to_string());

        let mut warning = self
            .session
            .get::<crate::model::Warning>("v3/warning/full", &params)?;

        if with_coastal_bulletin == Some(true) && crate::COASTAL_DEPARTMENT_LIST.contains(&domain) {
            let mut coastal = self.warning_full(&format!("{domain}10"), Some(false))?;
            warning
                .phenomenons_items
                .append(&mut coastal.phenomenons_items);
        }

        Ok(warning)
    }

    /**
     * Retrieve the thumbnail URL of the weather phenomenoms or alerts map.
     */
    pub fn warning_thumbnail(&self, domain: Option<&str>) -> String {
        format!(
            "{}/v3/warning/thumbnail?&token={}&domain={}",
            crate::session::METEOFRANCE_API_URL,
            crate::session::METEOFRANCE_API_TOKEN,
            domain.unwrap_or("france"),
        )
    }

    /**
     * Retrieves the meteorological dictionary from the Météo-France API.
     *
     * This dictionary includes information about various meteorological
     * phenomena and color codes used for weather warnings.
     */
    pub fn warning_dictionary(&self) -> crate::Result<crate::model::Dictionary> {
        let mut params = std::collections::HashMap::new();
        params.insert("lang", self.language.clone());

        self.session.get("v3/warning/dictionary", &params)
    }

    /**
     * Retrieve the picture of the day image URL & description.
     */
    pub fn picture_of_the_day(&self, domain: Option<&str>) -> crate::Result<crate::model::Picture> {
        let mut params = std::collections::HashMap::new();
        params.insert("domain", domain.unwrap_or("france").to_string());
        params.insert("report_type", "observation".to_string());
        params.insert("report_subtype", "image du jour".to_string());
        params.insert("format", "txt".to_string());

        let description: String = self.session.text("v2/report", &params)?;

        let picture = crate::model::Picture {
            image_url: format!(
                "{}/v2/report?domain={}&report_type=observation&report_subtype=image%20du%20jour&format=jpg&token={}",
                super::session::METEOFRANCE_API_URL,
                domain.unwrap_or("france"),
                super::session::METEOFRANCE_API_TOKEN,
            ),
            description,
        };

        Ok(picture)
    }
}
