markup::define! {
    Layout<Head: markup::Render, Main: markup::Render>(
        head: Head,
        main: Main,
    ) {
        @markup::doctype()
        html {
            head {
                @head
                style { @markup::raw(include_str!("main.css")) }
            }
            body {
                nav {
                    a[href = "/"] { "Home" }
                }
                main {
                    @main
                }
            }
        }
    }
}

#[rocket::get("/")]
fn get() -> rocket::response::content::RawHtml<String> {
    let template = Layout {
        head: markup::new! {
            title { "Home" }
        },
        main: markup::new! {
            h1 { "My contact form" }
            form[method = "post"] {
                label[for = "name"] { "Name" }
                input[id = "name", name = "name", type = "text", required];
                label[for = "message"] { "Message" }
                textarea[id = "message", name = "message", required, rows = 7] {}
                button[type = "submit"] { "Submit" }
            }
        },
    };

    rocket::response::content::RawHtml(template.to_string())
}

#[derive(rocket::FromForm)]
struct Contact {
    name: String,
    message: String,
}

#[rocket::post("/", data = "<form>")]
fn post(form: rocket::form::Form<Contact>) -> rocket::response::content::RawHtml<String> {
    let template = Layout {
        head: markup::new! {
            title { "Message sent! | Home" }
        },
        main: markup::new! {
            h1 { "Message sent!"}
            p {
                "Thanks for the "
                @form.message.chars().count()
                " character(s) long message, "
                strong { @form.name }
                "!"
            }
        },
    };

    rocket::response::content::RawHtml(template.to_string())
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![get, post])
}
