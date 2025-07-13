use axum::{extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};


pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn authenticate_request(
    headers: HeaderMap,
    request: Request,
    next: Next
) -> Result<Response, StatusCode> {
    // Logic for authenticating a request
    let Some(headers) = headers.get("Authorization") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let result = next.run(request).await;

    Ok(result)
}