use crate::environments::Environment;
use crate::errors::{Error, Kind};
use reqwest::Request;

pub struct Client<'a> {
    pub client_id: &'a str,
    pub secret: &'a str,
    pub public_key: &'a str,
    pub environment: Environment,
    pub http_client: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn call<T>(&self, endpoint: &str, body: &str) -> Result<T, Error>
    where
        for<'de> T: serde::de::Deserialize<'de>,
    {
        let request = self.new_request(endpoint, body);

        request.and_then(|req| self.execute_request::<T>(req))
    }

    fn new_request(&self, endpoint: &str, body: &str) -> Result<Request, Error> {
        let mut path = endpoint.to_string();

        if !endpoint.starts_with("/") {
            path.insert(0, '/');
        }

        let mut url = "https://".to_string();
        url.push_str(self.environment.host());
        url.push_str(path.as_str());

        let request = self
            .http_client
            .post(url.as_str())
            .body(body.to_string())
            .header("Content-Type", "application/json")
            .header("User-Agent", "Plaid Rust v0.0.1")
            .header("Plaid-Version", "2019-05-29");

        request
            .build()
            .map_err(|err| Error::new(Kind::Reqwest(err)))
    }

    fn execute_request<T>(&self, request: Request) -> Result<T, Error>
    where
        for<'de> T: serde::de::Deserialize<'de>,
    {
        self.http_client
            .execute(request)
            .map_err(|err| Error::new(Kind::Reqwest(err)))
            .and_then(|res| serde_json::from_reader(res).map_err(|err| Error::new(Kind::Json(err))))
    }
}
