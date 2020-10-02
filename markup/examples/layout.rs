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
        head: markup::new! {
            title { "Home" }
        },
        body: markup::new! {
            "This is the home page."
        },
    }
    .to_string()
}

fn contact() -> String {
    Layout {
        head: markup::new! {
            title { "Contact" }
        },
        body: markup::new! {
            "This is the contact page."
        },
    }
    .to_string()
}

fn main() {
    println!("{}", home());
    println!("{}", contact());
}
