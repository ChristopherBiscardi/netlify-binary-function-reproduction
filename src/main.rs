use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body,
    Error, Request, Response,
};

async fn function_handler(
    _event: Request,
) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::Text(
            r#"{message: "test hello"}"#.to_string(),
        ))?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
