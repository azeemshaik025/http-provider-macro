#[cfg(test)]
mod tests {
    use http_provider_macro::http_provider;
    use reqwest::{header::HeaderMap, Url};
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    // Define the provider with various endpoint configurations
    http_provider!(
        HttpProvider,
        {
            {
                path: "/users",
                method: GET,
                res: MyResponse,
            },
            {
                path: "/users/{id}",
                method: GET,
                path_params: PathParams,
                res: MyResponse,
            },
            {
                path: "/search",
                method: GET,
                query_params: QueryParams,
                res: MyResponse,
            },
            {
                path: "/data",
                method: GET,
                headers: HeaderMap,
                res: MyResponse,
            },
            {
                method: GET,
                res: MyResponse,
            },
            {
                path: "/users",
                method: POST,
                req: MyRequest,
                res: MyResponse,
            },
        }
    );

    // Test data structures
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct MyResponse {
        value: String,
    }

    #[derive(Serialize, Deserialize)]
    struct MyRequest {
        data: String,
    }

    #[derive(Serialize, Deserialize)]
    struct PathParams {
        id: String,
    }

    #[derive(Serialize, Deserialize)]
    struct QueryParams {
        q: String,
    }

    fn create_success_response(value: &str) -> MyResponse {
        MyResponse {
            value: value.to_string(),
        }
    }

    // Basic functionality tests
    #[tokio::test]
    async fn test_get_with_path() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let response = create_success_response("users");

        Mock::given(method("GET"))
            .and(wiremock::matchers::path("/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let provider = HttpProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));
        let result = provider.get_users().await?;

        assert_eq!(result.value, "users");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_with_path_params() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let response = create_success_response("user-123");

        Mock::given(method("GET"))
            .and(wiremock::matchers::path_regex(r"^/users/\w+$"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let provider = HttpProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));
        let result = provider
            .get_users_by_id(&PathParams {
                id: "123".to_string(),
            })
            .await?;

        assert_eq!(result.value, "user-123");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_with_query_params() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let response = create_success_response("search-result");

        Mock::given(method("GET"))
            .and(wiremock::matchers::path("/search"))
            .and(wiremock::matchers::query_param("q", "test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let provider = HttpProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));
        let result = provider
            .get_search(&QueryParams {
                q: "test".to_string(),
            })
            .await?;

        assert_eq!(result.value, "search-result");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_with_headers() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let response = create_success_response("with-headers");

        Mock::given(method("GET"))
            .and(wiremock::matchers::path("/data"))
            .and(wiremock::matchers::header("x-api-key", "secret"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let provider = HttpProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", "secret".parse()?);

        let result = provider.get_data(headers).await?;

        assert_eq!(result.value, "with-headers");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_without_path() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let response = create_success_response("no-path");

        Mock::given(method("GET"))
            .and(wiremock::matchers::path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let provider = HttpProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));
        let result = provider.get().await?;

        assert_eq!(result.value, "no-path");
        Ok(())
    }

    #[tokio::test]
    async fn test_post_with_body() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        let response = create_success_response("created");

        Mock::given(method("POST"))
            .and(wiremock::matchers::path("/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&mock_server)
            .await;

        let provider = HttpProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));
        let result = provider
            .post_users(&MyRequest {
                data: "test".to_string(),
            })
            .await?;

        assert_eq!(result.value, "created");
        Ok(())
    }

    // Trait-based mock provider test
    #[tokio::test]
    async fn test_trait_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
        // Simple provider for trait testing
        http_provider!(
            SimpleProvider,
            {
                {
                    path: "/items",
                    method: GET,
                    res: MyResponse,
                },
                {
                    path: "/items/{id}",
                    method: GET,
                    path_params: PathParams,
                    res: MyResponse,
                },
            }
        );

        struct MockProvider;

        impl SimpleProviderTrait for MockProvider {
            async fn get_items(&self) -> Result<MyResponse, SimpleProviderError> {
                Ok(create_success_response("mock-items"))
            }

            async fn get_items_by_id(
                &self,
                _path_params: &PathParams,
            ) -> Result<MyResponse, SimpleProviderError> {
                Ok(create_success_response("mock-item-123"))
            }
        }

        let mock = MockProvider;

        assert_eq!(mock.get_items().await?.value, "mock-items");
        assert_eq!(
            mock.get_items_by_id(&PathParams {
                id: "123".to_string()
            })
            .await?
            .value,
            "mock-item-123"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_optional_response() -> Result<(), Box<dyn std::error::Error>> {
        // Provider with optional response (no res field)
        http_provider!(
            NoResponseProvider,
            {
                {
                    path: "/delete",
                    method: DELETE,
                },
                {
                    path: "/update",
                    method: PUT,
                    req: MyRequest,
                },
            }
        );

        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(wiremock::matchers::path("/delete"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        Mock::given(method("PUT"))
            .and(wiremock::matchers::path("/update"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let provider = NoResponseProvider::new(Url::from_str(&mock_server.uri())?, Some(5000));

        // Test DELETE without response
        let result: Result<(), _> = provider.delete_delete().await;
        assert!(result.is_ok());

        // Test PUT without response
        let result: Result<(), _> = provider
            .put_update(&MyRequest {
                data: "test".to_string(),
            })
            .await;
        assert!(result.is_ok());

        Ok(())
    }
}
