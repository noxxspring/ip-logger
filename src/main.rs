use std::net::SocketAddr;

use axum::{extract::ConnectInfo, http::HeaderMap, response::{Html, IntoResponse}, routing::get, Router};


#[tokio::main]
async fn main() {

    // initialize logger (for logging to stdout)
    tracing_subscriber::fmt::init();

    //Define route and app
    let app = Router::new()
    .route("/", get(handler));

    //Bind to 0.0.0.0 so it can be accessible on render
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("🚀 Server running on http://{}", addr);

    // Start the server with ConnectionInfo enabled
    axum::Server::bind(&addr)
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
}

    //start handler that logs Visitor IP and returns a thank you page
    async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>, headers: HeaderMap, ) -> impl IntoResponse {

        // Extract real ip address from the X-Forwarded-For header

        let real_ip = headers
        .get("X-Forwarded-For")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).to_string()) // Take the first ip from the list
        .unwrap_or_else(|| addr.ip().to_string()); 

        println!("👤 New visitor IP: {}", real_ip);

        Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Thanks</title>
            <style>
                body { font-family: sans-serif; text-align: center; padding: 60px; }
                h1 { color: #3b82f6; }
            </style>
        </head>
        <body>
            <h1>✅ Thanks for visiting!</h1>
            <p>Your access has been recorded.</p>
        </body>
        </html>
    "#)
        
    }
    
