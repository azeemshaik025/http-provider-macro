//! Basic example demonstrating minimal usage of the `http_provider` macro.
//!
//! This example shows the simplest way to create an HTTP client provider
//! with just the essential fields: `method`, `path`, and `res`.

use http_provider_macro::http_provider;
use reqwest::Url;
use serde::Deserialize;

// Define your response types
#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    content: String,
}

// Define your HTTP provider with minimal configuration
http_provider!(
    ApiClient,
    {
        {
            path: "/users",
            method: GET,
            res: Vec<User>,
        },
        {
            path: "/posts",
            method: GET,
            res: Post,
        },
    }
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = Url::parse("https://api.example.com")?;
    let client = ApiClient::new(base_url, Some(5000));

    // Use the auto-generated methods
    let users = client.get_users().await?;
    println!("Found {} users", users.len());
    if let Some(user) = users.first() {
        println!("First user: #{} - {} ({})", user.id, user.name, user.email);
    }

    let post = client.get_posts().await?;
    println!("Post #{}: {} - {}", post.id, post.title, post.content);

    Ok(())
}
