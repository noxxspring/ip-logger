use std::{env, net::SocketAddr};

use axum::{extract::ConnectInfo, http::HeaderMap, response::{Html, IntoResponse}, routing::get, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Debug)]
struct IpInfoResponse {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: Option<String>,
}


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


        //Get Geolocation info using ipInfo api
        let location = get_geolocation(&real_ip).await;

        match location {
            Ok(location) => {
                let loc_string = location.loc.unwrap_or_default(); // latitude, logitude

                //Generate google map link
                let map_url = if !loc_string.is_empty() {
                    format!("https://www.google.com/maps?q={}", loc_string)
                }else{
                    "Location data unavailable".to_string()

                };

                println!(
                    "🌍 Location for IP {}: {}, {}, {}",
                    location.ip,
                    location.city.unwrap_or_default(),
                    location.region.unwrap_or_default(),
                    location.country.unwrap_or_default(),
                );
                println!("🗺️  Google Maps Link: {}", map_url)
            }
            Err(_e) => {
                println!("Error getting geolocation");
            }
        }

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
    

    // Function to get geolocation of the IP using ipinfo.io API
async fn get_geolocation(ip: &str) -> Result<IpInfoResponse, reqwest::Error> {
    // fetch the API key from the environment variable
    let api_key = env::var("IPINFO_API_KEY")
     .expect("IPINFO_API_KEY must be set in environment variables");

    let url = format!("https://ipinfo.io/{}/json?token={}", ip, api_key);
    let client = Client::new();
    let res = client.get(url).send().await?;
    let res = res.error_for_status()?;

    
    let location: IpInfoResponse = res.json().await?;

    Ok(location)
}
