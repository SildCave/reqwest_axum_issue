use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use reqwest::{multipart::Form, Client};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3222").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    
) -> impl IntoResponse {
    let client = Client::new();
    let form = Form::new()
        .text("response", "0.0K406IAnuz23vNPIs82poR_43g6Nv_tPApjHnOqu4a7kkT-aaqYaee1gx-SY2Wz6EhyyF0U9EVCRKOZvRdBZBU9WtEiMEU0JuTjcPicrnWlUcNdIl1-Gj8VnqGrkNCPSgUYQmCFb4T1IEDHjhbr3xSJoO5_-U6Rg0RQ8FqGEn4mp07HL8VLncuZdrsT2jprHy73fl3Hmf48-FCah1BpFCzOrGbqiC_LPRBnQf1zd_mVtxo0dsvWkc8Cgqx7oBi_N7CDKvROE_COoMkBVPv1bdRXJQIP11ajPIhv4fxqNR_wvd_e2tXrMvN3pYVVF-i0kRKWO9xd_u7-vQBThi2QaZ0YlHnfOWgEwMTAuw1cQuWQBSXWNZ1LSahivQp7quvBojgINiUYeW9hRyVg-zSQFnzkaRVjAadtDj6jIuJP02Gy7mmeNQ-Y6BJlc01kxx95nJxGVQCQa0fa151iA9PIjq5Pg6x8dn3IqswHBDtreFxvSIWvcatvgY3r0ssN-OTlupKsrqmV5J6nwKtqpi_GgXhoNDwmfQDOMekLYFcN3SjuxpRVM4vY40PnILLeV6eesz95PqqAJE8kJWMuH90uO17ZOQiigmZnl6eoyPxC6dnqRbIEVu5226IhfFdhm6Vl0h85ILBeksJuDUSG4qHA1Y4G0COdCreF5x4EzYi7JsftxoweTNA_OYMSWFpHRQ1Us._cZY1T0QO92C0W-kASzScg.fa7534f5d9b2a1857d67e345a9737b3b27d635caf52160d2398779b54f50d6fd")
        .text("secret", "2x0000000000000000000000000000000AA");

    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

    // imposter

    let response = client.post(url)
        .multipart(form)
        .send()
        .await;

    if response.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response();
    }
    //let response = response.unwrap();
    println!("{:?}", response);

    //
    (StatusCode::OK, "ok").into_response()
}

