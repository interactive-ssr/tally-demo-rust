use maud::{html, Markup, DOCTYPE};

use axum::{
    routing::get,
    Router, response::Html, extract::Form,
};
use serde::Deserialize;

static mut COUNT: i32 = 0;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/tally", get(tally).post(tally));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Args {
    addx: Option<String>,
    subx: Option<String>
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

async fn tally(form: Option<Form<Args>>) -> Html<String> {
    if let Some(form) = form {
        let args = form.0;
        if let Some(addx) = args.addx {
            if !addx.is_empty() {
                unsafe {
                    COUNT = COUNT + 1;
                }
            }
        }
        if let Some(subx) = args.subx {
            if !subx.is_empty() {
                unsafe {
                    COUNT = COUNT - 1;
                }
            }
        } 
    }
    unsafe {
        let count = COUNT.to_string();
        let markup = html! {
            (DOCTYPE)
            head {}
            body {
                h1 { "Tally" }
                button onclick="rr(this)" action="addx" { "+" }
                button onclick="rr(this)" action="subx" { "-" }
                (natural_number_list(COUNT))
            }
        };
        Html(markup.into_string())
    }

}

