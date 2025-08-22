use crate::network::client::Client;
use crate::types;

impl Client {
    // GetRequest sends a GET Request to the server including the authorization token
    pub async fn get_request(&self, url: &String) -> Result<types::Response, reqwest::Error> {
        // FIXME client registered testen

        let auth_token = self.auth_token.lock().await.clone();
        let echo_json: types::Response = self
            .http_client
            .get(url)
            .header("Authorization", auth_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }

    // DeleteRequest sends a DELETE Request to delete the client out of the server
    // including the authorization token
    pub async fn delete_request(
        &self,
        url: &String,
        body: String,
    ) -> Result<types::Response, reqwest::Error> {
        let auth_token: String = self.auth_token.lock().await.clone();
        let echo_json: types::Response = self
            .http_client
            .delete(url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }

    // PostReqeust sends a Post Request to send a message to the server
    // including the authorization token
    pub async fn post_request(
        &self,
        url: &String,
        body: String,
    ) -> Result<types::Response, reqwest::Error> {
        let auth_token: String = self.auth_token.lock().await.clone();
        let echo_json: types::Response = self
            .http_client
            .post(url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }
}
