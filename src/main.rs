
use axum::{middleware, routing::get, Json, Router};
use axum::http::{Request, Response};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse};
use axum::body::Body;
use serde::{Serialize};
use MadsVejrApp::getLowestTemp;

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = //tokio TCP listener
        tokio::net::TcpListener::bind("0.0.0.0:7878")//localhost:7878/
            .await
            .expect("Failed to bind tcp listener");

    axum::serve(listener, app)//start server
        .await.expect("Failed to run server");
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(getIndex))
        .layer(middleware::from_fn(getAssets))
        .layer(middleware::from_fn(fetchWeather))
}

//end-point should not be sending pure html... To be changed with future updates
async fn getIndex() -> Html<String> {
    let file = tokio::fs::read_to_string("assets/views/index.html").await.unwrap();
    Html(file)
}
//middleware - catch the request for scripts/CSS here
async fn getAssets(
    request: Request<Body>,
    next: Next) -> Response<Body>
{
    if request.uri().path().contains("assets/") {
        let requestPath = request.uri().path().split_at(1).1;
        let file = tokio::fs::read(requestPath).await.unwrap();
        return Response::new(Body::from(file));
    }
    next.run(request).await //if nothing is requested from the assets folder, then continue
}

async fn fetchWeather(
    request: Request<Body>,
    next: Next) -> Response<Body>
{
    if (request.uri().path().contains("fetchWeather/")) {
        let weatherData = getWeatherData().await;
        return weatherData.into_response();

    }
    next.run(request).await
}

async fn getWeatherData() -> Json<Data> {
    let weatherData = getLowestTemp().await;
    let dataStruct = Data{
        temp: weatherData.0,
        coord: weatherData.1,
    };
    Json(dataStruct)
}
#[derive(Serialize)]
struct Data{
    temp: f32,
    coord: [f32; 2],
}
