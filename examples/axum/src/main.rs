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

async fn get() -> impl axum::response::IntoResponse {
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

    axum::response::Html(template.to_string())
}

#[derive(serde::Deserialize)]
struct Contact {
    name: String,
    message: String,
}

async fn post(form: axum::extract::Form<Contact>) -> impl axum::response::IntoResponse {
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

    axum::response::Html(template.to_string())
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(get).post(post));

    eprintln!("starting on http://0.0.0.0:3000");

    axum::Server::bind(&([0, 0, 0, 0], 3000).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
