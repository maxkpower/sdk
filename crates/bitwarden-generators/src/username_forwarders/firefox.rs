use reqwest::{
    header::{self},
    StatusCode,
};

use crate::username::UsernameError;

pub async fn generate(
    http: &reqwest::Client,
    api_token: String,
    website: Option<String>,
) -> Result<String, UsernameError> {
    generate_with_api_url(http, api_token, website, "https://relay.firefox.com".into()).await
}

async fn generate_with_api_url(
    http: &reqwest::Client,
    api_token: String,
    website: Option<String>,
    api_url: String,
) -> Result<String, UsernameError> {
    #[derive(serde::Serialize)]
    struct Request {
        enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        generated_for: Option<String>,
        description: String,
    }

    let description = super::format_description_ff(&website);

    let response = http
        .post(format!("{api_url}/api/v1/relayaddresses/"))
        .header(header::AUTHORIZATION, format!("Token {api_token}"))
        .json(&Request {
            enabled: true,
            generated_for: website,
            description,
        })
        .send()
        .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(UsernameError::InvalidApiKey);
    }

    // Throw any other errors
    response.error_for_status_ref()?;

    #[derive(serde::Deserialize)]
    struct Response {
        full_address: String,
    }
    let response: Response = response.json().await?;

    Ok(response.full_address)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::username::UsernameError;

    #[tokio::test]
    async fn test_mock_success() {
        use wiremock::{matchers, Mock, ResponseTemplate};

        let server = wiremock::MockServer::start().await;

        server
            .register(
                Mock::given(matchers::path("/api/v1/relayaddresses/"))
                    .and(matchers::method("POST"))
                    .and(matchers::header("Content-Type", "application/json"))
                    .and(matchers::header("Authorization", "Token MY_TOKEN"))
                    .and(matchers::body_json(json!({
                        "enabled": true,
                        "generated_for": "example.com",
                        "description": "example.com - Generated by Bitwarden."
                    })))
                    .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                        "full_address": "ofuj4d4qw@mozmail.com"
                    })))
                    .expect(1),
            )
            .await;

        let address = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_TOKEN".into(),
            Some("example.com".into()),
            format!("http://{}", server.address()),
        )
        .await
        .unwrap();
        assert_eq!(address, "ofuj4d4qw@mozmail.com");

        server.verify().await;
    }

    #[tokio::test]
    async fn test_mock_without_website() {
        use wiremock::{matchers, Mock, ResponseTemplate};

        let server = wiremock::MockServer::start().await;

        server
            .register(
                Mock::given(matchers::path("/api/v1/relayaddresses/"))
                    .and(matchers::method("POST"))
                    .and(matchers::header("Content-Type", "application/json"))
                    .and(matchers::header("Authorization", "Token MY_OTHER_TOKEN"))
                    .and(matchers::body_json(json!({
                        "enabled": true,
                        "description": "Generated by Bitwarden."
                    })))
                    .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                        "full_address": "856f7765@mozmail.com"
                    })))
                    .expect(1),
            )
            .await;

        let address = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_OTHER_TOKEN".into(),
            None,
            format!("http://{}", server.address()),
        )
        .await
        .unwrap();
        assert_eq!(address, "856f7765@mozmail.com");

        server.verify().await;
    }

    #[tokio::test]
    async fn test_mock_invalid_token() {
        use wiremock::{matchers, Mock, ResponseTemplate};

        let server = wiremock::MockServer::start().await;

        server
            .register(
                Mock::given(matchers::path("/api/v1/relayaddresses/"))
                    .and(matchers::method("POST"))
                    .and(matchers::header("Content-Type", "application/json"))
                    .and(matchers::header("Authorization", "Token MY_FAKE_TOKEN"))
                    .and(matchers::body_json(json!({
                        "enabled": true,
                        "generated_for": "example.com",
                        "description": "example.com - Generated by Bitwarden."
                    })))
                    .respond_with(ResponseTemplate::new(401))
                    .expect(1),
            )
            .await;

        let error = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_FAKE_TOKEN".into(),
            Some("example.com".into()),
            format!("http://{}", server.address()),
        )
        .await
        .unwrap_err();

        assert_eq!(error.to_string(), UsernameError::InvalidApiKey.to_string());

        server.verify().await;
    }
}