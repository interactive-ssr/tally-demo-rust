use maud::{html, Markup};

use axum::{
    routing::get,
    Router, response::Html,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/tally", get(tally).post(tally));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


struct Args {
    addx: Option<String>,
    subx: Option<String>
}

async fn tally() -> Html<String> {
    let markup = html! {
        body {
            h1 { "Tally" }
            button onclick="rr(this)" action="addx" { "+" }
            button onclick="rr(this)" action="subx" { "-" }

        }
    };
    Html(markup.into_string())
}

