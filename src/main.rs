#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::borrow::Cow;
use regex::Regex;
use serde::Serialize;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::request::Form;

fn clarify(s: &str) -> Cow<str> {
    let re = Regex::new(r"(\[|\]|BLOCKED)").unwrap();
    re.replace_all(s, "")
}

#[derive(FromForm, Debug)]
struct Obfuscated {
    field_obfuscated_link: String,
    field_proto: String,
}

#[post("/", data = "<clarify_form>")]
fn sink(clarify_form: Form<Obfuscated>) -> Template {

    #[derive(Serialize)]
    struct Context {
        clarified_link: String,
    }

    let values: Obfuscated = clarify_form.into_inner();

    let context;
    if values.field_proto == "None" {
        context = Context {
            clarified_link: format!("{}", 
                clarify(&values.field_obfuscated_link))
        };
    } else {
        context = Context {
            clarified_link: format!("{}://{}", values.field_proto,
                                    clarify(&values.field_obfuscated_link))
        };
    }
    Template::render("home", context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![sink])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
