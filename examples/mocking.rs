//! Example demonstrating how to mock the generated HTTP provider for testing.
//!
//! The `http_provider` macro generates a trait (e.g., `ApiClientTrait`) that
//! the provider struct implements. You can implement this trait yourself to
//! create mock providers for testing without making actual HTTP requests.

use http_provider_macro::http_provider;
use serde::{Deserialize, Serialize};

// Response type
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
}

// Path parameters
#[derive(Serialize)]
struct UserPathParams {
    id: u32,
}

// Define the provider with a single endpoint
http_provider!(
    ApiClient,
    {
        {
            path: "/users/{id}",
            method: GET,
            path_params: UserPathParams,
            res: User,
        },
    }
);

// Mock provider implementing the generated trait
struct MockProvider;

impl ApiClientTrait for MockProvider {
    async fn get_users_by_id(&self, path_params: &UserPathParams) -> Result<User, ApiClientError> {
        Ok(User {
            id: path_params.id,
            name: format!("User {}", path_params.id),
        })
    }
}

// Function that works with any provider implementing the trait
async fn get_user_name<P: ApiClientTrait>(
    provider: &P,
    user_id: u32,
) -> Result<String, ApiClientError> {
    let user = provider
        .get_users_by_id(&UserPathParams { id: user_id })
        .await?;
    Ok(user.name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the mock provider
    let mock = MockProvider;
    let user = mock.get_users_by_id(&UserPathParams { id: 42 }).await?;
    println!("Mock user: {:?}", user);

    // Use the trait in a function
    let name = get_user_name(&mock, 42).await?;
    println!("User name: {}", name);

    // The same function works with the real provider too:
    // let base_url = reqwest::Url::parse("https://api.example.com")?;
    // let real_client = ApiClient::new(base_url, Some(5000));
    // let name = get_user_name(&real_client, 42).await?;

    Ok(())
}
