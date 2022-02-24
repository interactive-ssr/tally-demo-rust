#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use maud::{html, Markup, DOCTYPE};
use rocket::{request::Form, response::content::Html};

pub type Session<'a> = rocket_session::Session<'a, i32>;

fn main() -> () {
    rocket::ignite()
        .attach(Session::fairing())
        .mount("/", routes![tally, tally_get])
        .launch();
}

#[derive(FromForm)]
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

#[get("/tally")]
fn tally_get(session: Session) -> Html<String> {
    tally(None, session)
}

#[post("/tally", data = "<args>")]
fn tally(args: Option<Form<Args>>, session: Session) -> Html<String> {
    if let Some(args) = args {
        if let Some(addx) = &args.addx {
            if !addx.is_empty() {
                session.tap(|n| {
                    *n += 1;
                });
            }
        }
        if let Some(subx) = &args.subx {
            if !subx.is_empty() {
                session.tap(|n| {
                    if *n >= 1 {
                        *n -= 1;
                    }
                });
            }
        }
    }
    let count = session.tap(|n| *n);

    let markup = html! {
        (DOCTYPE)
        head {}
        body {
            h1 { "Tally" }
            button onclick="rr(this)" action="addx" { "+" }
            button onclick="rr(this)" action="subx" { "-" }
            (natural_number_list(count))
        }
    };
    Html(markup.into_string())
}
