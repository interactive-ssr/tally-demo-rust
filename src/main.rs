use maud::html;
use rocket::response::content::Html;
use rocket::form::Form;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Html<String> {
    let name = "charles";
    let markup = html! {
        h1 { "Hi, " (name) "!" }
    };
    Html(markup.into_string())
}

static mut COUNT: i32 = 0;

#[derive(FromForm)]
struct Args {
    addx: Option<String>,
    subx: Option<String>
}

#[get("/tally")]
fn tally_get() -> Html<String> {
    tally(None)
}

#[post("/tally", data = "<args>")]
fn tally(args: Option<Form<Args>>) -> Html<String> {

    let markup = html! {
        body {
            h1 { "Tally" }
            button onclick="rr(this)" action="addx" { "+" }
            button onclick="rr(this)" action="subx" { "-" }

        }
    };
    Html(markup.into_string())

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, tally, tally_get])
}