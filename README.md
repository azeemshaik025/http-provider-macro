# HTTP Provider Macro

Generate type-safe HTTP client methods from endpoint definitions.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
http-provider-macro = "0.1.3"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

Define your endpoints:

```rust
use http_provider_macro::http_provider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Serialize)]
struct UserPath {
    id: u32,
}

http_provider!(
    UserApi,
    {
        {
            path: "/users",
            method: GET,
            res: Vec<User>,
        },
        {
            path: "/users/{id}",
            method: GET,
            path_params: UserPath,
            res: User,
        },
    }
);

// Use it
let client = UserApi::new(reqwest::Url::parse("https://api.example.com")?, Some(5000));
let users = client.get_users().await?;
let user = client.get_users_by_id(&UserPath { id: 1 }).await?;
```

## Endpoint Fields

**Required:**

- `method`: HTTP method (GET, POST, PUT, DELETE)
- `res`: Response type (optional, defaults to `()`)

**Optional:**

- `path`: URL path (e.g., "/users/{id}")
- `path_params`: Type for path parameters
- `query_params`: Type for query parameters
- `req`: Request body type
- `headers`: Header type (e.g., `reqwest::header::HeaderMap`)
- `fn_name`: Custom function name

## Examples

See the `examples/` directory:

- `basic.rs` - Simple usage
- `params.rs` - Path and query parameters
- `advanced.rs` - All features
- `mocking.rs` - Testing with traits

## License

MIT OR Apache-2.0
