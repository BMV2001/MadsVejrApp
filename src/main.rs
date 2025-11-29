
use axum::{routing::get, Router};
use axum::response::Html;
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
        .route("/", get(getWeather))
}

//end-point should not be sending pure html... To be changed with future updates
async fn getWeather() -> Html<String> {
    let weatherData = getLowestTemp().await;
    let html : String = format!("<body>
                                    <p>Så du kunne godt tænke dig at se vejret hva? Du er da kommet lidt på afveje</p>
                                    <h1>MEN FRYGT EJ!</h1>
                                    <p>Jeg har bikset en lille hjemmeside op til dig - Den viser godt nok kun den laveste temperatur i Danmark samt koordinatsættet lol</p>
                                    <p>Det er altså {}C</p>
                                    <p>på koordinatsættet {},{}</p>
                                    <p>(men altså du kan da lige få et link til google Maps)</p>
                                    <a href=\"https://www.google.com/maps/place/{},{}\">(DU SKAL KLIKKE HER!)</a>
                                    <p>... og inden du spørger: </p>
                                    <h1>HvOrFoR Er dEr InTeT NaVn pÅ KoOrDinAtByEn???</h1>
                                    <p>...Jeg er doven, derfor linket til google :) </p>
                                </body>"
           , weatherData.0, weatherData.1[0], weatherData.1[1], weatherData.1[0], weatherData.1[1]);
    Html(html)
}