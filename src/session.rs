pub(crate) static METEOFRANCE_API_URL: &str = "https://webservice.meteofrance.com";
pub(crate) static METEOFRANCE_API_TOKEN: &str = "__Wj7dVSTjV9YGu1guveLyDq0g7S7TfTjaHBTPTpO0kj8__";

pub(crate) struct Session {
    access_token: String,
    client: reqwest::blocking::Client,
}

impl Session {
    pub fn new(access_token: Option<&str>) -> Self {
        Self {
            access_token: access_token.unwrap_or(METEOFRANCE_API_TOKEN).to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get<T>(
        &self,
        path: &str,
        params: &std::collections::HashMap<&str, String>,
    ) -> crate::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut query = params.clone();
        query.insert("token", self.access_token.clone());

        self.request(reqwest::Method::GET, path, &query)
    }

    fn request<T>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: &std::collections::HashMap<&str, String>,
    ) -> crate::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let url = format!("{METEOFRANCE_API_URL}/{path}");

        let response = self.client.request(method, url).query(&query).send()?;

        if response.status().is_success() {
            response.json().map_err(crate::Error::from)
        } else {
            Err(crate::Error::Service(response.json()?))
        }
    }

    pub fn text(
        &self,
        path: &str,
        params: &std::collections::HashMap<&str, String>,
    ) -> crate::Result<String> {
        let mut query = params.clone();
        query.insert("token", self.access_token.clone());

        let url = format!("{METEOFRANCE_API_URL}/{path}");

        let response = self
            .client
            .request(reqwest::Method::GET, url)
            .query(&query)
            .send()?;

        if response.status().is_success() {
            dbg!(response.text()).map_err(crate::Error::from)
        } else {
            Err(crate::Error::Service(crate::model::Error {
                error: response.status().as_u16() as u32,
                message: response.text()?,
            }))
        }
    }
}
