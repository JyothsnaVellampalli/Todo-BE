use axum:: {
  middleware::Next,
  extract::Request,
};


pub async fn logging_middleware(
    req: Request,
    next: Next,
) -> axum::response::Response {
    println!("[{}] Request made to: {}", req.method(), req.uri());
    let response = next.run(req).await;
    response
}