use maud::{html, Markup, DOCTYPE};
use rocket::{response::content::Html, form::Form};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![tally, tally_get])
}

static mut COUNT: i32 = 0;

#[derive(FromForm)]
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

#[get("/tally")]
fn tally_get() -> Html<String> {
    tally(None)
}

#[post("/tally", data = "<args>")]
fn tally(args: Option<Form<Args>>) -> Html<String> {
    if let Some(args) = args {
        if let Some(addx) = &args.addx {
            if !addx.is_empty() {
                unsafe {
                    COUNT = COUNT + 1;
                }
            }
        }
        if let Some(subx) = &args.subx {
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

