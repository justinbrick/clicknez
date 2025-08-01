use axum::{extract::{FromRequestParts, Request}, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};

use crate::auth;


pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl<S> FromRequestParts<S> for Identity where S: Send + Sync {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self,Self::Rejection> {

        let authorization = parts.headers.get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;


        // Extract the user information from the authorization header
        Ok(Identity {
            first_name: "John".into(),
            last_name: "Doe".into(),
            email: "john.doe@example.com".into(),
        })
    }
}
