markup::define! {
    Layout<Head: markup::Render, Body: markup::Render>(
        head: Head,
        body: Body,
    ) {
        @markup::doctype()
        html {
            head {
                @head
            }
            body {
                @body
            }
        }
    }
}

fn home() -> String {
    Layout {
        head: markup::dynamic! {
            title { "Home" }
        },
        body: markup::dynamic! {
            "This is the home page."
        },
    }
    .to_string()
}

fn contact() -> String {
    Layout {
        head: markup::dynamic! {
            title { "Contact" }
        },
        body: markup::dynamic! {
            "This is the contact page."
        },
    }
    .to_string()
}

fn main() {
    println!("{}", home());
    println!("{}", contact());
}
