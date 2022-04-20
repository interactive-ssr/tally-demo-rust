use axum_database_sessions::{
    axum_session_runner, AxumSession, AxumSessionConfig, AxumSessionStore, AxumDatabasePool,
};
use maud::{html, Markup, DOCTYPE};

use axum::{extract::Form, response::Html, routing::get, Router};
use serde::Deserialize;

static mut COUNT: i32 = 0;

#[tokio::main]
async fn main() {
    // session setup
    let session_config = AxumSessionConfig::default().with_table_name("test_table");
    let session_store = AxumSessionStore::new(None, session_config);

    // build our application with a single route
    let app = Router::new()
        .route("/tally", get(tally).post(tally))
        .layer(axum_session_runner!(session_store))
        .layer(tower_cookies::CookieManagerLayer::new());

    println!("Starting...");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Args {
    addx: Option<String>,
    subx: Option<String>,
}

fn natural_number_list(end: i32) -> Markup {
    let range = 1..=end;
    html! {
        ul {
            @for num in range {
                li { (num) }
            }
        }
    }
}

async fn tally(session: AxumSession, form: Option<Form<Args>>) -> Html<String> {
    let mut count = session.get("count").await.unwrap_or(0);
    if let Some(form) = form {
        let args = form.0;
        if let Some(addx) = args.addx {
            if !addx.is_empty() {
                println!("increment");
                count += 1;
            }
        }
        if let Some(subx) = args.subx {
            if !subx.is_empty() {
                println!("decrement");
                count -= 1;
            }
        }
        session.set("count", count).await;
    }
    let markup = html! {
        (DOCTYPE)
        head {}
        body {
            h1 { "Tally" }
            button onclick="rr(this)" action="addx" { "+" }
            button onclick="rr(this)" action="subx" { "-" }
            (natural_number_list(0))
        }
    };
    Html(markup.into_string())
}
