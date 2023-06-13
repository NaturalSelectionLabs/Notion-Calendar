use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    http::HeaderValue,
    Body, Client, HeaderMap, Method, Request, Uri,
};
use hyper_tls::HttpsConnector;
use std::env;

use crate::database::{DatabaseEntry, DatabaseResponse};

pub async fn list_calendars() -> Result<Vec<DatabaseEntry>, Box<dyn std::error::Error>> {
    let api_token = env::var("NOTION_API_KEY")?;
    let database_id = env::var("NOTION_DATABASE_ID")?;

    let endpoint_url = format!("https://api.notion.com/v1/databases/{}/query", database_id);

    let uri: Uri = endpoint_url.parse()?;

    // Set up the request headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Notion-Version", "2022-06-28".parse()?);

    // Send the GET request to retrieve database data
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(Body::default())
        .expect("request builder");

    req.headers_mut().extend(headers);

    let response = client.request(req).await?;

    // Read the response body as bytes
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();

    // Convert the response body bytes to a string
    let body_str = String::from_utf8_lossy(&body_bytes);

    // Parse and handle the response
    let response_json: DatabaseResponse = serde_json::from_str(&body_str)?;

    Ok(response_json.results)
}
