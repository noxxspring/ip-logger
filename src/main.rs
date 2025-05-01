use std::net::SocketAddr;

use axum::{extract::ConnectInfo, response::{Html, IntoResponse}, routing::get, Router};


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
    async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        // Log the IP Address of the user 
        println!("👤 New visitor IP: {}", addr.ip());

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
    
