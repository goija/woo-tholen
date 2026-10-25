use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Vang de browser CORS preflight direct succesvol af
    if req.method() == "OPTIONS" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::Empty)?);
    }

    // Retourneer uw standaard JSON/GeoJSON
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text(r#"{"type":"FeatureCollection","features":[]}"#.to_string()))?)
}
