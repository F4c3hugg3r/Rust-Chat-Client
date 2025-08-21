use crate::network::client::Client;

impl<'a> Client<'a> {
    // GetRequest sends a GET Request to the server including the authorization token
    pub async fn get_request(&self, url: &String) -> Result<reqwest::Response, reqwest::Error> {
        // FIXME auf das auth_token darf nur mit lock zugegriffen werden und client registered testen

        let echo_json = self
            .http_client
            .get(url)
            .header("Authorization", &self.auth_token)
            .send()
            .await?;
        Ok(echo_json)
    }

    // DeleteRequest sends a DELETE Request to delete the client out of the server
    // including the authorization token
    pub async fn delete_request(
        &self,
        url: &String,
        body: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // FIXME auf das auth_token darf nur mit lock zugegriffen werden

        let echo_json = self
            .http_client
            .delete(url)
            .header("Authorization", &self.auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        Ok(echo_json)
    }

    // PostReqeust sends a Post Request to send a message to the server
    // including the authorization token
    pub async fn post_request(
        &self,
        url: &String,
        body: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // FIXME auf das auth_token darf nur mit lock zugegriffen werden

        let echo_json = self
            .http_client
            .post(url)
            .header("Authorization", &self.auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        Ok(echo_json)
    }
}
