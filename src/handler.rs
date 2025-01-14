use axum::{
    body::{Body, Bytes}, 
    extract::{Request, State}, 
    middleware::Next, 
    response::{IntoResponse, Response}, 
    Json
};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, fmt::Debug};
use tracing::debug;
use http_body_util::BodyExt;
use crate::reporter::Reporter;

#[derive(Clone)]
pub struct ReportHandler {
    pub reporter: Reporter,
}

#[derive(Serialize, Deserialize, Debug, Clone, strum_macros::Display)]
pub enum Stage {
    RouterRequest,
    RouterResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoprocessorRequest {
    pub id: String,
    pub version: u16,
    pub stage: Stage,
    pub headers: Option<HashMap<String, Vec<String>>>,
    pub context: Option<CoprocessorRequestContext>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoprocessorRequestContext {
    pub entries: HashMap<String, Value>,
}

#[debug_handler]
pub async fn handler(State(report_handler): State<ReportHandler>, Json(body): Json<CoprocessorRequest>) -> impl IntoResponse {
    debug!("Received request: {:?}", body);

    let reporter = report_handler.reporter.clone();
    let response = json!({
            "id": body.id,
            "version": body.version,
            "stage": body.stage,
            "control": "continue",
        });

    tokio::task::spawn(async move {
        reporter.add_report(body).await;
    });

    (
        http::StatusCode::OK, 
        Json(response),
    )
}

// This middleware logs the request and response bodies for debugging purposes.
pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
